
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Workspace Validation
//!
//! Path: `src/toolchain/workspace/validation.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Validates the consistency of an explicitly discovered workspace,
//! its resolved members, their project manifests, and their inspected
//! source layouts.
//!
//! Validation is deliberately separate from discovery, member resolution,
//! layout inspection, manifest parsing, metadata fingerprinting,
//! dependency resolution, compilation, and execution.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Workspace-level consistency validation.
//! - Validation policy options.
//! - Structured workspace validation issues.
//! - Validation severity and stable issue codes.
//! - Validation reports.
//! - Cross-checks between discovery, members, and layouts.
//! - Detection of duplicate source-file ownership across members.
//!
//! This module does not own:
//!
//! - Workspace discovery.
//! - Manifest parsing or interpretation.
//! - Workspace member declaration parsing or expansion.
//! - Project identity allocation.
//! - Source-layout discovery or traversal implementation.
//! - Source-language parsing or semantic analysis.
//! - Dependency resolution.
//! - Metadata fingerprints.
//! - Compiler or backend invocation.
//! - Target selection or hardware discovery.
//! - Physical resource allocation.
//! - Runtime or distributed scheduling.
//!
//! ## Integration
//!
//! - `super::discovery` supplies `DiscoveryResult` and manifest identity.
//! - `super::members` supplies validated `WorkspaceMembers`.
//! - `super::layout` supplies inspected `ProjectLayout` values.
//! - `super::metadata` may consume a successfully validated workspace.
//! - `crate::toolchain::ids` owns project and workspace identities.
//! - `crate::toolchain::errors` owns structured error classification.
//! - `crate::toolchain::result` owns the canonical result type.
//! - `crate::zamani_project_config` remains the manifest authority.
//!
//! ## Validation semantics
//!
//! A successful validation report means that all checks requested by
//! the selected validation options passed against the inspected snapshot.
//!
//! It does not mean that:
//!
//! - Source code is syntactically valid.
//! - Source code is semantically valid.
//! - Dependencies have been resolved.
//! - A compiler or backend is available.
//! - A target is feasible.
//! - Physical resources have been allocated.
//! - An artifact has been produced.
//! - A program can execute.
//!
//! Those checks belong to their respective authoritative subsystems.
//!
//! ## Scalability
//!
//! This module introduces no arbitrary maximum for:
//!
//! - Workspace members.
//! - Projects.
//! - Source files.
//! - Directory depth.
//! - Source formats.
//! - Computing domains.
//! - Targets.
//! - Hardware resources.
//! - Distributed participants.
//!
//! Validation memory is proportional to the member and source-path
//! indexes required to detect cross-member inconsistencies.
//!
//! Actual operation remains subject to available memory, address space,
//! filesystem capabilities, operating-system permissions, and storage.
//!
//! Literal infinite storage or execution is not promised.
//!
//! ## Security
//!
//! - No unsafe Rust.
//! - No shell or external process execution.
//! - No manifest content execution.
//! - No automatic filesystem modification.
//! - No acceptance of paths outside their owning roots.
//! - No following symbolic links when validating manifest files.
//! - Source traversal uses the existing layout implementation, which
//!   does not follow directory symlinks.
//! - Filesystem failures are not converted into successful validation.
//!
//! Portable standard filesystem APIs cannot eliminate every hostile
//! time-of-check/time-of-use race. Adversarial shared filesystems require
//! appropriate filesystem permissions or an OS-specific handle-relative
//! filesystem boundary.

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};
use crate::toolchain::ids::{
    ProjectId,
    WorkspaceId,
};
use crate::toolchain::result::Result;

use super::discovery::{
    DiscoveryResult,
    ManifestKind,
};
use super::layout::ProjectLayout;
use super::members::{
    WorkspaceMember,
    WorkspaceMembers,
};

use std::collections::{
    BTreeMap,
    BTreeSet,
};
use std::fs;
use std::path::{
    Component,
    Path,
    PathBuf,
};

// ============================================================================
// VALIDATION POLICY
// ============================================================================

/// Options controlling workspace consistency validation.
///
/// The defaults validate structural consistency without imposing a
/// requirement that every workspace contain a source file or a member.
///
/// Empty workspaces and projects with externally generated or
/// provider-supplied source inputs can therefore be valid.
///
/// Applications may opt into stricter requirements explicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkspaceValidationOptions {
    require_members: bool,
    require_source_files: bool,
}

impl Default for WorkspaceValidationOptions {
    fn default() -> Self {
        Self {
            require_members: false,
            require_source_files: false,
        }
    }
}

impl WorkspaceValidationOptions {
    /// Construct the default validation policy.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            require_members: false,
            require_source_files: false,
        }
    }

    /// Require at least one resolved workspace member.
    ///
    /// This is a project policy, not a universal workspace restriction.
    #[must_use]
    pub const fn with_require_members(
        mut self,
        required: bool,
    ) -> Self {
        self.require_members = required;
        self
    }

    /// Require at least one discovered source file across all members.
    ///
    /// This should be enabled only when the consuming workflow requires
    /// source files to exist before its next stage.
    ///
    /// Projects whose inputs are generated or supplied by registered
    /// providers may legitimately have no local source files.
    #[must_use]
    pub const fn with_require_source_files(
        mut self,
        required: bool,
    ) -> Self {
        self.require_source_files = required;
        self
    }

    /// Whether at least one workspace member is required.
    #[must_use]
    pub const fn require_members(self) -> bool {
        self.require_members
    }

    /// Whether at least one source file is required.
    #[must_use]
    pub const fn require_source_files(self) -> bool {
        self.require_source_files
    }
}

// ============================================================================
// ISSUE SEVERITY
// ============================================================================

/// Severity of a workspace validation issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValidationSeverity {
    /// The workspace violates a required structural invariant.
    Error,

    /// The workspace is structurally valid but may need attention.
    Warning,
}

impl ValidationSeverity {
    /// Return whether this severity makes a report invalid.
    #[must_use]
    pub const fn prevents_success(self) -> bool {
        matches!(self, Self::Error)
    }
}

// ============================================================================
// STABLE ISSUE CODES
// ============================================================================

/// Stable classification of workspace validation issues.
///
/// These codes are machine-readable identifiers. Do not change the
/// meaning of an existing code when extending this enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum WorkspaceValidationCode {
    /// Discovery did not identify an explicit workspace.
    MissingExplicitWorkspace,

    /// The discovered workspace root does not match the resolved members.
    WorkspaceRootMismatch,

    /// The workspace manifest is missing, invalid, or outside the root.
    InvalidWorkspaceManifest,

    /// A workspace has no members when policy requires members.
    NoMembers,

    /// A member identity appears more than once.
    DuplicateProjectIdentity,

    /// A member directory is outside the workspace root.
    MemberOutsideWorkspace,

    /// A member's resolved directory no longer matches its declaration.
    MemberRootMismatch,

    /// A member manifest is invalid or outside its project root.
    InvalidMemberManifest,

    /// A member has no corresponding project layout.
    MissingProjectLayout,

    /// More than one layout was supplied for one project identity.
    DuplicateProjectLayout,

    /// A layout was supplied for a project that is not a workspace member.
    UnexpectedProjectLayout,

    /// A layout belongs to a different project root.
    ProjectLayoutRootMismatch,

    /// A project has no source files when policy requires source files.
    NoSourceFiles,

    /// A source file is outside its owning project.
    SourceOutsideProject,

    /// A source file is owned by more than one workspace member.
    DuplicateSourceOwnership,
}

impl WorkspaceValidationCode {
    /// Return the stable machine-readable code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingExplicitWorkspace => "ZWV0001",
            Self::WorkspaceRootMismatch => "ZWV0002",
            Self::InvalidWorkspaceManifest => "ZWV0003",
            Self::NoMembers => "ZWV0004",
            Self::DuplicateProjectIdentity => "ZWV0005",
            Self::MemberOutsideWorkspace => "ZWV0006",
            Self::MemberRootMismatch => "ZWV0007",
            Self::InvalidMemberManifest => "ZWV0008",
            Self::MissingProjectLayout => "ZWV0009",
            Self::DuplicateProjectLayout => "ZWV0010",
            Self::UnexpectedProjectLayout => "ZWV0011",
            Self::ProjectLayoutRootMismatch => "ZWV0012",
            Self::NoSourceFiles => "ZWV0013",
            Self::SourceOutsideProject => "ZWV0014",
            Self::DuplicateSourceOwnership => "ZWV0015",
        }
    }
}

// ============================================================================
// VALIDATION ISSUE
// ============================================================================

/// One structured workspace validation issue.
///
/// `subject` is descriptive context, such as a project identity.
/// `path` is an optional filesystem path associated with the issue.
///
/// These fields are diagnostic context; they do not replace canonical
/// project or workspace identities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceValidationIssue {
    severity: ValidationSeverity,
    code: WorkspaceValidationCode,
    message: String,
    subject: Option<String>,
    path: Option<PathBuf>,
}

impl WorkspaceValidationIssue {
    fn new(
        severity: ValidationSeverity,
        code: WorkspaceValidationCode,
        message: impl Into<String>,
        subject: Option<String>,
        path: Option<PathBuf>,
    ) -> Self {
        Self {
            severity,
            code,
            message: message.into(),
            subject,
            path,
        }
    }

    /// Return the issue severity.
    #[must_use]
    pub const fn severity(&self) -> ValidationSeverity {
        self.severity
    }

    /// Return the stable issue classification.
    #[must_use]
    pub const fn code(&self) -> WorkspaceValidationCode {
        self.code
    }

    /// Return the stable machine-readable issue code.
    #[must_use]
    pub const fn code_str(&self) -> &'static str {
        self.code.as_str()
    }

    /// Return the human-readable explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the descriptive subject, if present.
    #[must_use]
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }

    /// Return the associated path, if present.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Result of validating one workspace snapshot.
///
/// The report contains all structural issues that could be checked
/// without encountering an underlying filesystem or implementation error.
///
/// An error returned by [`validate_workspace`] means that validation
/// could not be completed reliably. It must not be interpreted as a
/// successful or partially successful report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceValidationReport {
    workspace_id: WorkspaceId,
    workspace_root: PathBuf,
    member_count: usize,
    source_file_count: usize,
    issues: Vec<WorkspaceValidationIssue>,
}

impl WorkspaceValidationReport {
    fn new(
        workspace_id: WorkspaceId,
        workspace_root: PathBuf,
        member_count: usize,
        source_file_count: usize,
        issues: Vec<WorkspaceValidationIssue>,
    ) -> Self {
        Self {
            workspace_id,
            workspace_root,
            member_count,
            source_file_count,
            issues,
        }
    }

    /// Return the validated workspace identity.
    #[must_use]
    pub fn workspace_id(&self) -> &WorkspaceId {
        &self.workspace_id
    }

    /// Return the canonical workspace root.
    #[must_use]
    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    /// Return the number of resolved workspace members inspected.
    #[must_use]
    pub const fn member_count(&self) -> usize {
        self.member_count
    }

    /// Return the number of distinct source files inspected.
    #[must_use]
    pub const fn source_file_count(&self) -> usize {
        self.source_file_count
    }

    /// Borrow all validation issues in deterministic validation order.
    #[must_use]
    pub fn issues(&self) -> &[WorkspaceValidationIssue] {
        &self.issues
    }

    /// Return whether the workspace passed all required checks.
    ///
    /// Warnings do not make a report invalid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.issues.iter().any(|issue| {
            issue.severity().prevents_success()
        })
    }

    /// Return whether the report contains no issues.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.issues.is_empty()
    }

    /// Return whether the report contains errors.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|issue| {
            issue.severity() == ValidationSeverity::Error
        })
    }

    /// Return whether the report contains warnings.
    #[must_use]
    pub fn has_warnings(&self) -> bool {
        self.issues.iter().any(|issue| {
            issue.severity() == ValidationSeverity::Warning
        })
    }

    /// Count issues of the requested severity.
    #[must_use]
    pub fn count_by_severity(
        &self,
        severity: ValidationSeverity,
    ) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity() == severity)
            .count()
    }
}

// ============================================================================
// PUBLIC VALIDATION API
// ============================================================================

/// Validate a discovered workspace, its resolved members, and their layouts.
///
/// `discovery` must be the result of the existing workspace discovery
/// subsystem. `members` must be the result of the existing workspace
/// member resolver. `layouts` must contain one inspected layout for every
/// resolved member.
///
/// The function cross-checks these independently produced snapshots.
/// It does not parse manifests or rediscover workspace members.
///
/// # Validation performed
///
/// - An explicit workspace was discovered.
/// - Discovery and member resolution agree on the workspace root.
/// - The discovered workspace manifest is a regular file inside the root.
/// - The workspace manifest is classified as workspace-bearing.
/// - Member identities are unique.
/// - Member roots remain inside the workspace root.
/// - Member declarations still resolve to their recorded roots.
/// - Member manifests remain regular files inside their project roots.
/// - Every member has exactly one matching project layout.
/// - No layout belongs to an unrelated project.
/// - Every layout belongs to its corresponding member root.
/// - Source files remain inside their owning project roots.
/// - A source file is not claimed by multiple workspace members.
/// - Optional member and source-file requirements are enforced.
///
/// # Failure behavior
///
/// Structural inconsistencies are returned in a validation report.
/// Filesystem failures that prevent reliable inspection are returned as
/// `Err(ToolchainError)`.
///
/// The caller must check [`WorkspaceValidationReport::is_valid`] before
/// proceeding to metadata collection, dependency resolution, or building.
///
/// # Portability
///
/// This function does not select a target, inspect hardware, establish
/// computational feasibility, or claim that a project can execute on
/// a particular machine.
pub fn validate_workspace(
    discovery: &DiscoveryResult,
    members: &WorkspaceMembers,
    layouts: &[(ProjectId, ProjectLayout)],
) -> Result<WorkspaceValidationReport> {
    validate_workspace_with_options(
        discovery,
        members,
        layouts,
        WorkspaceValidationOptions::default(),
    )
}

/// Validate a workspace using an explicit validation policy.
///
/// See [`validate_workspace`] for the full validation contract.
pub fn validate_workspace_with_options(
    discovery: &DiscoveryResult,
    members: &WorkspaceMembers,
    layouts: &[(ProjectId, ProjectLayout)],
    options: WorkspaceValidationOptions,
) -> Result<WorkspaceValidationReport> {
    let mut issues = Vec::new();

    // ------------------------------------------------------------------------
    // Workspace discovery and root consistency
    // ------------------------------------------------------------------------

    if !discovery.has_explicit_workspace() {
        push_issue(
            &mut issues,
            WorkspaceValidationCode::MissingExplicitWorkspace,
            "workspace validation requires an explicitly discovered workspace",
            None,
            None,
        );

        return Ok(WorkspaceValidationReport::new(
            members.workspace_id().clone(),
            members.workspace_root().to_path_buf(),
            members.len(),
            0,
            issues,
        ));
    }

    let discovered_root = discovery
        .workspace_root()
        .ok_or_else(|| {
            ToolchainError::internal(
                "discovery reports an explicit workspace but has no workspace root",
            )
            .with_operation("workspace.validation"),
        })?;

    let canonical_root = canonical_directory(
        discovered_root,
        "workspace.validation.canonicalize_workspace_root",
    )?;

    let members_root = canonical_directory(
        members.workspace_root(),
        "workspace.validation.canonicalize_members_root",
    )?;

    if canonical_root != members_root {
        push_issue(
            &mut issues,
            WorkspaceValidationCode::WorkspaceRootMismatch,
            "discovery and resolved workspace members refer to different canonical workspace roots",
            Some(members.workspace_id().to_string()),
            Some(members_root.clone()),
        );
    }

    // Use the member collection's root as the ownership root only after
    // establishing that it is the same canonical root as discovery.
    // On mismatch, continue collecting useful diagnostics against the
    // discovered root, but the report will remain invalid.
    let workspace_root = canonical_root;

    // ------------------------------------------------------------------------
    // Workspace manifest
    // ------------------------------------------------------------------------

    let discovered_manifest = discovery
        .workspace_manifest()
        .ok_or_else(|| {
            ToolchainError::internal(
                "discovery reports an explicit workspace without a workspace manifest",
            )
            .with_operation("workspace.validation"),
        })?;

    if !discovered_manifest.kind().has_workspace()
        || !matches!(
            discovered_manifest.kind(),
            ManifestKind::Workspace | ManifestKind::PackageAndWorkspace
        )
    {
        push_issue(
            &mut issues,
            WorkspaceValidationCode::InvalidWorkspaceManifest,
            "the discovered manifest is not classified as workspace-bearing",
            Some(members.workspace_id().to_string()),
            Some(discovered_manifest.path().to_path_buf()),
        );
    }

    let workspace_manifest = validate_regular_file(
        discovered_manifest.path(),
        &workspace_root,
        "workspace.validation.workspace_manifest",
    )?;

    if workspace_manifest.is_none() {
        push_issue(
            &mut issues,
            WorkspaceValidationCode::InvalidWorkspaceManifest,
            "the workspace manifest is missing, is not a regular non-symbolic-link file, or is outside the workspace root",
            Some(members.workspace_id().to_string()),
            Some(discovered_manifest.path().to_path_buf()),
        );
    }

    // ------------------------------------------------------------------------
    // Workspace member policy
    // ------------------------------------------------------------------------

    if options.require_members() && members.is_empty() {
        push_issue(
            &mut issues,
            WorkspaceValidationCode::NoMembers,
            "the selected validation policy requires at least one workspace member",
            Some(members.workspace_id().to_string()),
            Some(workspace_root.clone()),
        );
    }

    // ------------------------------------------------------------------------
    // Build a deterministic project-layout index
    // ------------------------------------------------------------------------

    let mut layout_index: BTreeMap<ProjectId, &ProjectLayout> =
        BTreeMap::new();

    for (project_id, layout) in layouts {
        if layout_index
            .insert(project_id.clone(), layout)
            .is_some()
        {
            push_issue(
                &mut issues,
                WorkspaceValidationCode::DuplicateProjectLayout,
                "more than one project layout was supplied for the same project identity",
                Some(project_id.to_string()),
                Some(layout.project_root().to_path_buf()),
            );
        }
    }

    let member_ids: BTreeSet<ProjectId> = members
        .iter()
        .map(|member| member.project_id().clone())
        .collect();

    for (project_id, layout) in &layout_index {
        if !member_ids.contains(project_id) {
            push_issue(
                &mut issues,
                WorkspaceValidationCode::UnexpectedProjectLayout,
                "a project layout was supplied for an identity that is not a resolved workspace member",
                Some(project_id.to_string()),
                Some(layout.project_root().to_path_buf()),
            );
        }
    }

    // ------------------------------------------------------------------------
    // Member and source validation
    // ------------------------------------------------------------------------

    let mut seen_member_ids = BTreeSet::new();
    let mut seen_member_roots = BTreeSet::new();

    // The path index detects a physical source file being attributed to
    // more than one logical workspace project.
    let mut source_owners: BTreeMap<PathBuf, ProjectId> =
        BTreeMap::new();

    let mut source_file_count = 0usize;

    for member in members.iter() {
        let project_id = member.project_id();

        if !seen_member_ids.insert(project_id.clone()) {
            push_issue(
                &mut issues,
                WorkspaceValidationCode::DuplicateProjectIdentity,
                "a project identity occurs more than once in the resolved workspace members",
                Some(project_id.to_string()),
                Some(member.canonical_root().to_path_buf()),
            );
        }

        // Verify that the original declaration is a relative path and
        // contains no parent traversal, root, or platform-prefix component.
        if !is_safe_member_relative_path(member.relative_path()) {
            push_issue(
                &mut issues,
                WorkspaceValidationCode::MemberOutsideWorkspace,
                "the member declaration is not a safe workspace-relative path",
                Some(project_id.to_string()),
                Some(member.relative_path().to_path_buf()),
            );

            continue;
        }

        let declared_root = workspace_root.join(member.relative_path());

        let canonical_member_root = match canonical_directory(
            member.canonical_root(),
            "workspace.validation.canonicalize_member_root",
        ) {
            Ok(path) => path,
            Err(error) => return Err(error),
        };

        if !canonical_member_root.starts_with(&workspace_root) {
            push_issue(
                &mut issues,
                WorkspaceValidationCode::MemberOutsideWorkspace,
                "the canonical member directory is outside the workspace root",
                Some(project_id.to_string()),
                Some(canonical_member_root.clone()),
            );

            continue;
        }

        if !seen_member_roots.insert(canonical_member_root.clone()) {
            push_issue(
                &mut issues,
                WorkspaceValidationCode::MemberRootMismatch,
                "multiple workspace members resolve to the same canonical project directory",
                Some(project_id.to_string()),
                Some(canonical_member_root.clone()),
            );
        }

        let canonical_declared_root = match canonical_directory(
            &declared_root,
            "workspace.validation.resolve_member_declaration",
        ) {
            Ok(path) => path,
            Err(error) => {
                // A member may have been removed or replaced after member
                // resolution. Report the stale snapshot as invalid rather
                // than claiming that the workspace is still consistent.
                push_issue(
                    &mut issues,
                    WorkspaceValidationCode::MemberRootMismatch,
                    format!(
                        "the declared member directory can no longer be resolved: {error}"
                    ),
                    Some(project_id.to_string()),
                    Some(declared_root),
                );

                continue;
            }
        };

        if canonical_declared_root != canonical_member_root {
            push_issue(
                &mut issues,
                WorkspaceValidationCode::MemberRootMismatch,
                "the member declaration no longer resolves to the recorded canonical project directory",
                Some(project_id.to_string()),
                Some(declared_root),
            );

            continue;
        }

        // --------------------------------------------------------------------
        // Member manifest
        // --------------------------------------------------------------------

        let member_manifest = validate_regular_file(
            member.manifest_path(),
            &canonical_member_root,
            "workspace.validation.member_manifest",
        )?;

        if member_manifest.is_none() {
            push_issue(
                &mut issues,
                WorkspaceValidationCode::InvalidMemberManifest,
                "the member manifest is missing, is not a regular non-symbolic-link file, or is outside its project root",
                Some(project_id.to_string()),
                Some(member.manifest_path().to_path_buf()),
            );
        }

        // --------------------------------------------------------------------
        // Matching project layout
        // --------------------------------------------------------------------

        let layout = match layout_index.remove(project_id) {
            Some(layout) => layout,

            None => {
                push_issue(
                    &mut issues,
                    WorkspaceValidationCode::MissingProjectLayout,
                    "the resolved workspace member has no corresponding inspected project layout",
                    Some(project_id.to_string()),
                    Some(canonical_member_root.clone()),
                );

                continue;
            }
        };

        let layout_root = canonical_directory(
            layout.project_root(),
            "workspace.validation.canonicalize_layout_root",
        )?;

        if layout_root != canonical_member_root {
            push_issue(
                &mut issues,
                WorkspaceValidationCode::ProjectLayoutRootMismatch,
                "the inspected project layout belongs to a different canonical project root",
                Some(project_id.to_string()),
                Some(layout_root),
            );

            continue;
        }

        // --------------------------------------------------------------------
        // Source-file traversal
        // --------------------------------------------------------------------

        let mut member_source_count = 0usize;

        layout.visit_source_files(|source_path| {
            let source_path = source_path.to_path_buf();

            let canonical_source = validate_regular_file(
                &source_path,
                &canonical_member_root,
                "workspace.validation.source_file",
            )?;

            let Some(canonical_source) = canonical_source else {
                push_issue(
                    &mut issues,
                    WorkspaceValidationCode::SourceOutsideProject,
                    "a discovered source path is not a regular non-symbolic-link file contained by its owning project",
                    Some(project_id.to_string()),
                    Some(source_path),
                );

                return Ok(());
            };

            if let Some(previous_owner) = source_owners.get(&canonical_source) {
                if previous_owner != project_id {
                    push_issue(
                        &mut issues,
                        WorkspaceValidationCode::DuplicateSourceOwnership,
                        "the same canonical source file is included in more than one workspace member",
                        Some(format!(
                            "{} and {}",
                            previous_owner,
                            project_id,
                        )),
                        Some(canonical_source),
                    );
                }
            } else {
                source_owners.insert(
                    canonical_source,
                    project_id.clone(),
                );
            }

            member_source_count = member_source_count
                .checked_add(1)
                .ok_or_else(|| {
                    ToolchainError::resource_exhausted(
                        "source-file count exceeded the process address-space representation",
                    )
                    .with_operation("workspace.validation.count_sources")
                })?;

            Ok(())
        })?;

        source_file_count = source_file_count
            .checked_add(member_source_count)
            .ok_or_else(|| {
                ToolchainError::resource_exhausted(
                    "workspace source-file count exceeded the process address-space representation",
                )
                .with_operation("workspace.validation.count_sources")
            })?;

        if options.require_source_files() && member_source_count == 0 {
            push_issue(
                &mut issues,
                WorkspaceValidationCode::NoSourceFiles,
                "the selected validation policy requires this project to contain at least one discovered source file",
                Some(project_id.to_string()),
                Some(canonical_member_root),
            );
        }
    }

    // Layouts remaining in the index belong to identities not successfully
    // consumed as workspace members. Report them explicitly.
    for (project_id, layout) in layout_index {
        push_issue(
            &mut issues,
            WorkspaceValidationCode::UnexpectedProjectLayout,
            "an inspected project layout was not consumed by a resolved workspace member",
            Some(project_id.to_string()),
            Some(layout.project_root().to_path_buf()),
        );
    }

    Ok(WorkspaceValidationReport::new(
        members.workspace_id().clone(),
        workspace_root,
        members.len(),
        source_file_count,
        issues,
    ))
}

// ============================================================================
// ISSUE CONSTRUCTION
// ============================================================================

fn push_issue(
    issues: &mut Vec<WorkspaceValidationIssue>,
    code: WorkspaceValidationCode,
    message: impl Into<String>,
    subject: Option<String>,
    path: Option<PathBuf>,
) {
    issues.push(WorkspaceValidationIssue::new(
        ValidationSeverity::Error,
        code,
        message,
        subject,
        path,
    ));
}

// ============================================================================
// PATH VALIDATION
// ============================================================================

fn is_safe_member_relative_path(path: &Path) -> bool {
    if path.as_os_str().is_empty() || path.is_absolute() {
        return false;
    }

    let mut has_component = false;

    for component in path.components() {
        match component {
            Component::Normal(_) => {
                has_component = true;
            }

            // The member resolver permits `.` as an explicit declaration
            // of the workspace root itself.
            Component::CurDir if path == Path::new(".") => {
                has_component = true;
            }

            Component::CurDir
            | Component::ParentDir
            | Component::RootDir
            | Component::Prefix(_) => {
                return false;
            }
        }
    }

    has_component
}

fn canonical_directory(
    path: &Path,
    operation: &'static str,
) -> Result<PathBuf> {
    let canonical = fs::canonicalize(path).map_err(|source| {
        ToolchainError::from_io(operation, source)
            .with_context(path.display().to_string())
    })?;

    let metadata = fs::symlink_metadata(&canonical).map_err(|source| {
        ToolchainError::from_io(operation, source)
            .with_context(canonical.display().to_string())
    })?;

    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                "workspace or project root is not a regular directory",
            )
            .with_operation(operation)
            .with_context(canonical.display().to_string()),
        );
    }

    Ok(canonical)
}

/// Validate a regular non-symbolic-link file and its containment.
///
/// Returns `Ok(None)` for a structural invalidity that should be represented
/// in the validation report. Returns `Err` when the filesystem cannot be
/// inspected reliably.
fn validate_regular_file(
    path: &Path,
    owning_root: &Path,
    operation: &'static str,
) -> Result<Option<PathBuf>> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,

        Err(error)
            if error.kind() == std::io::ErrorKind::NotFound =>
        {
            return Ok(None);
        }

        Err(source) => {
            return Err(
                ToolchainError::from_io(operation, source)
                    .with_context(path.display().to_string()),
            );
        }
    };

    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Ok(None);
    }

    let canonical = fs::canonicalize(path).map_err(|source| {
        ToolchainError::from_io(operation, source)
            .with_context(path.display().to_string())
    })?;

    if !canonical.starts_with(owning_root) {
        return Ok(None);
    }

    // Recheck the resolved path. This detects ordinary replacement and
    // type-change races, but is not a race-free hostile-filesystem
    // transaction.
    let final_metadata =
        fs::symlink_metadata(&canonical).map_err(|source| {
            ToolchainError::from_io(operation, source)
                .with_context(canonical.display().to_string())
        })?;

    if final_metadata.file_type().is_symlink()
        || !final_metadata.is_file()
    {
        return Ok(None);
    }

    Ok(Some(canonical))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_options_do_not_require_members_or_source_files() {
        let options = WorkspaceValidationOptions::default();

        assert!(!options.require_members());
        assert!(!options.require_source_files());
    }

    #[test]
    fn validation_options_can_require_members() {
        let options = WorkspaceValidationOptions::new()
            .with_require_members(true);

        assert!(options.require_members());
        assert!(!options.require_source_files());
    }

    #[test]
    fn validation_options_can_require_source_files() {
        let options = WorkspaceValidationOptions::new()
            .with_require_source_files(true);

        assert!(!options.require_members());
        assert!(options.require_source_files());
    }

    #[test]
    fn validation_options_can_be_combined() {
        let options = WorkspaceValidationOptions::new()
            .with_require_members(true)
            .with_require_source_files(true);

        assert!(options.require_members());
        assert!(options.require_source_files());
    }

    #[test]
    fn issue_codes_are_stable_and_distinct() {
        let codes = [
            WorkspaceValidationCode::MissingExplicitWorkspace,
            WorkspaceValidationCode::WorkspaceRootMismatch,
            WorkspaceValidationCode::InvalidWorkspaceManifest,
            WorkspaceValidationCode::NoMembers,
            WorkspaceValidationCode::DuplicateProjectIdentity,
            WorkspaceValidationCode::MemberOutsideWorkspace,
            WorkspaceValidationCode::MemberRootMismatch,
            WorkspaceValidationCode::InvalidMemberManifest,
            WorkspaceValidationCode::MissingProjectLayout,
            WorkspaceValidationCode::DuplicateProjectLayout,
            WorkspaceValidationCode::UnexpectedProjectLayout,
            WorkspaceValidationCode::ProjectLayoutRootMismatch,
            WorkspaceValidationCode::NoSourceFiles,
            WorkspaceValidationCode::SourceOutsideProject,
            WorkspaceValidationCode::DuplicateSourceOwnership,
        ];

        let unique: BTreeSet<&str> = codes
            .iter()
            .map(|code| code.as_str())
            .collect();

        assert_eq!(unique.len(), codes.len());
    }

    #[test]
    fn member_relative_path_accepts_normal_paths() {
        assert!(is_safe_member_relative_path(
            Path::new("applications/example")
        ));

        assert!(is_safe_member_relative_path(
            Path::new("libraries/core")
        ));
    }

    #[test]
    fn member_relative_path_accepts_explicit_workspace_root() {
        assert!(is_safe_member_relative_path(Path::new(".")));
    }

    #[test]
    fn member_relative_path_rejects_absolute_paths() {
        assert!(!is_safe_member_relative_path(
            Path::new("/outside/project")
        ));
    }

    #[test]
    fn member_relative_path_rejects_parent_traversal() {
        assert!(!is_safe_member_relative_path(
            Path::new("../outside")
        ));

        assert!(!is_safe_member_relative_path(
            Path::new("applications/../../outside")
        ));
    }

    #[test]
    fn member_relative_path_rejects_embedded_current_directory() {
        assert!(!is_safe_member_relative_path(
            Path::new("applications/./example")
        ));
    }

    #[test]
    fn severity_success_semantics_are_explicit() {
        assert!(
            ValidationSeverity::Error.prevents_success()
        );

        assert!(
            !ValidationSeverity::Warning.prevents_success()
        );
    }
}
