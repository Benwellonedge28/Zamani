
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Clean Command
//!
//! Path: `src/toolchain/commands/clean.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! ## Purpose
//!
//! Coordinate removal of recognized, toolchain-generated resources.
//!
//! This command supports:
//!
//! - Generated build outputs.
//! - Compiler and backend caches.
//! - Other generated resources registered by trusted toolchain providers.
//!
//! It supports dry runs, structured reports, cancellation, provider failures,
//! idempotent removal, and resource-specific ownership validation.
//!
//! ## Ownership
//!
//! This module owns clean-command orchestration and reporting.
//!
//! It does not own:
//!
//! - Project or workspace discovery.
//! - Manifest parsing.
//! - Build-output generation.
//! - Artifact identity or artifact storage.
//! - Compiler cache implementation.
//! - Backend cache implementation.
//! - Incremental dependency graphs.
//! - External process execution.
//! - Package installation or registry storage.
//! - Runtime state or physical hardware.
//!
//! Those responsibilities remain with their canonical repository owners.
//!
//! ## Safety contract
//!
//! The command never guesses that a file is generated from its filename,
//! extension, directory name, or location.
//!
//! A `CleanProvider` must:
//!
//! - Discover only resources it can positively identify as generated.
//! - Restrict discovery to the requested canonical project root.
//! - Exclude source files, manifests, lockfiles, user data, credentials,
//!   installed dependencies, registry data and unrelated files.
//! - Revalidate resource ownership and identity immediately before removal.
//! - Avoid following symlinks or traversing outside the authorized root.
//! - Treat changed or ambiguous resources as skipped.
//! - Make removal idempotent.
//! - Preserve the original cause of provider failures.
//!
//! If a provider cannot establish ownership or safe deletion, it must fail
//! closed. The command must never convert uncertainty into permission.
//!
//! ## Cache integration
//!
//! The canonical `BackendCache::clear()` operation may be used by a provider
//! when the requested cache scope authorizes clearing that cache.
//!
//! `IncrementalBuildCoordinator::forget_build()` may be used to forget
//! tracked build state. It does not delete external artifact files.
//!
//! Compiler artifact storage must be cleaned through its own storage
//! authority, not by deleting paths inferred from artifact metadata.
//!
//! ## Scalability
//!
//! No arbitrary project-size, artifact-count, target-count, domain-count,
//! cache-count, or hardware-count limit is introduced.
//!
//! Candidate discovery is streamed through a visitor. Report memory grows
//! with the number of reported entries and is subject to actual available
//! memory. Allocation failure is reported rather than silently truncating
//! the clean operation.
//!
//! Literal infinite storage or deletion is not promised.
//!
//! ## POCO-REAF
//!
//! Cleaning is independent of computational domain. A provider may identify
//! generated resources from classical, quantum, HDL, AI, hybrid, distributed,
//! networking, cryptographic, embedded, accelerator, or future build flows.
//!
//! No domain-specific deletion logic is hard-coded here.

use serde::{Deserialize, Serialize};

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::result::ToolchainResult;

/// Stable schema identifier for clean reports.
pub const CLEAN_REPORT_SCHEMA: &str = "zamani.toolchain.clean-report.v1";

/// Stable operation identifier for structured errors.
const OPERATION: &str = "commands.clean";

fn invalid_input(message: impl Into<String>) -> ToolchainError {
    ToolchainError::invalid_input(message).with_operation(OPERATION)
}

fn validation_error(message: impl Into<String>) -> ToolchainError {
    ToolchainError::validation_failed(message).with_operation(OPERATION)
}

fn allocation_error(
    subject: &'static str,
    error: std::collections::TryReserveError,
) -> ToolchainError {
    ToolchainError::resource_exhausted(format!(
        "unable to reserve memory for {subject}: {error}"
    ))
    .with_operation(OPERATION)
}

fn io_error(operation: &'static str, error: std::io::Error) -> ToolchainError {
    ToolchainError::from_io(operation, error)
}

// ============================================================================
// CLEAN SCOPE
// ============================================================================

/// Selects which classes of recognized generated resources may be cleaned.
///
/// The provider remains responsible for identifying the actual resources
/// belonging to each scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanScope {
    /// Generated build outputs belonging to the requested project.
    BuildOutputs,

    /// Recognized compiler, backend, and toolchain caches.
    Caches,

    /// All recognized generated resources within the requested project.
    AllRecognizedGenerated,
}

// ============================================================================
// REQUEST
// ============================================================================

/// Immutable request to clean generated resources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleanRequest {
    root: PathBuf,
    scope: CleanScope,
    dry_run: bool,
}

impl CleanRequest {
    /// Create a clean request for the supplied project or workspace root.
    ///
    /// The root must exist and resolve to a directory when the command runs.
    ///
    /// The default scope is all recognized generated resources.
    /// The default mode performs the requested operation; use
    /// [`Self::with_dry_run`] to inspect the plan without removing anything.
    pub fn new(root: impl Into<PathBuf>) -> ToolchainResult<Self> {
        let root = root.into();

        if root.as_os_str().is_empty() {
            return Err(invalid_input("clean root must not be empty"));
        }

        Ok(Self {
            root,
            scope: CleanScope::AllRecognizedGenerated,
            dry_run: false,
        })
    }

    /// Select the resources that may be cleaned.
    #[must_use]
    pub const fn with_scope(mut self, scope: CleanScope) -> Self {
        self.scope = scope;
        self
    }

    /// Select whether this request only reports what would be removed.
    #[must_use]
    pub const fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Return the requested project or workspace root.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Return the selected clean scope.
    #[must_use]
    pub const fn scope(&self) -> CleanScope {
        self.scope
    }

    /// Return whether this request is a dry run.
    #[must_use]
    pub const fn is_dry_run(&self) -> bool {
        self.dry_run
    }

    /// Resolve and validate the root before provider discovery.
    fn resolve_root(&self) -> ToolchainResult<Self> {
        let canonical_root = std::fs::canonicalize(&self.root)
            .map_err(|error| io_error("clean.resolve_root", error))?;

        let metadata = std::fs::metadata(&canonical_root)
            .map_err(|error| io_error("clean.inspect_root", error))?;

        if !metadata.is_dir() {
            return Err(invalid_input(format!(
                "clean root is not a directory: {}",
                canonical_root.display()
            )));
        }

        Ok(Self {
            root: canonical_root,
            scope: self.scope,
            dry_run: self.dry_run,
        })
    }
}

// ============================================================================
// EXTENSIBLE RESOURCE KIND
// ============================================================================

/// Open-ended identity for a kind of generated resource.
///
/// This is deliberately a string-backed identifier rather than a closed
/// enumeration of computing domains, vendors, cache implementations or
/// artifact formats.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CleanResourceKind(String);

impl CleanResourceKind {
    /// Construct a resource-kind identifier.
    pub fn new(value: impl Into<String>) -> ToolchainResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(invalid_input(
                "clean resource kind must not be empty",
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(invalid_input(
                "clean resource kind must not contain control characters",
            ));
        }

        Ok(Self(value))
    }

    /// Return the canonical resource-kind identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// CANDIDATE
// ============================================================================

/// A resource positively identified by a trusted clean provider.
///
/// The resource locator is opaque to the command. It may identify a
/// filesystem object, a cache namespace, an artifact-store object, or
/// another provider-owned generated resource.
///
/// The command does not interpret the locator as a filesystem path.
///
/// A provider must revalidate the ownership evidence and revision when
/// removing the resource. A stale candidate must not delete a replacement
/// resource that appeared after discovery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleanCandidate {
    provider_id: String,
    resource_id: String,
    kind: CleanResourceKind,
    root: PathBuf,
    location: String,
    revision: String,
    estimated_bytes: Option<u64>,
}

impl CleanCandidate {
    /// Construct a candidate from a provider's verified inventory.
    ///
    /// `root` must be the canonical root supplied to provider discovery.
    ///
    /// `provider_id` identifies the provider.
    ///
    /// `resource_id` is a stable, provider-local resource identity.
    ///
    /// `revision` identifies the discovered resource version or ownership
    /// evidence. The provider must compare it again before removal.
    ///
    /// `location` is display-only metadata; it is never interpreted as a
    /// deletion path by this command.
    pub fn new(
        provider_id: impl Into<String>,
        resource_id: impl Into<String>,
        kind: CleanResourceKind,
        root: impl Into<PathBuf>,
        location: impl Into<String>,
        revision: impl Into<String>,
        estimated_bytes: Option<u64>,
    ) -> ToolchainResult<Self> {
        let provider_id = provider_id.into();
        let resource_id = resource_id.into();
        let root = root.into();
        let location = location.into();
        let revision = revision.into();

        if provider_id.trim().is_empty() {
            return Err(invalid_input("clean provider identity must not be empty"));
        }

        if resource_id.trim().is_empty() {
            return Err(invalid_input("clean resource identity must not be empty"));
        }

        if !root.is_absolute() {
            return Err(invalid_input(
                "clean candidate root must be an absolute path",
            ));
        }

        if location.trim().is_empty() {
            return Err(invalid_input(
                "clean candidate display location must not be empty",
            ));
        }

        if revision.trim().is_empty() {
            return Err(invalid_input(
                "clean candidate revision evidence must not be empty",
            ));
        }

        if provider_id.chars().any(char::is_control)
            || resource_id.chars().any(char::is_control)
            || location.chars().any(char::is_control)
            || revision.chars().any(char::is_control)
        {
            return Err(invalid_input(
                "clean candidate identifiers must not contain control characters",
            ));
        }

        Ok(Self {
            provider_id,
            resource_id,
            kind,
            root,
            location,
            revision,
            estimated_bytes,
        })
    }

    /// Return the provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    /// Return the provider-local resource identity.
    #[must_use]
    pub fn resource_id(&self) -> &str {
        &self.resource_id
    }

    /// Return the open-ended resource kind.
    #[must_use]
    pub fn kind(&self) -> &CleanResourceKind {
        &self.kind
    }

    /// Return the canonical root associated with this candidate.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Return the display-only resource location.
    #[must_use]
    pub fn location(&self) -> &str {
        &self.location
    }

    /// Return the revision evidence supplied by the provider.
    #[must_use]
    pub fn revision(&self) -> &str {
        &self.revision
    }

    /// Return the provider's optional size estimate.
    #[must_use]
    pub const fn estimated_bytes(&self) -> Option<u64> {
        self.estimated_bytes
    }

    fn identity(&self) -> (&str, &str) {
        (&self.provider_id, &self.resource_id)
    }
}

// ============================================================================
// DISCOVERY CONTROL
// ============================================================================

/// Controls whether a provider continues candidate discovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanVisitControl {
    /// Continue visiting candidates.
    Continue,

    /// Stop visiting candidates without treating the stop as a failure.
    Stop,
}

/// A provider's result when removing one recognized resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanRemovalOutcome {
    /// The resource was removed successfully.
    Removed {
        /// Actual number of bytes removed, if known.
        bytes_removed: Option<u64>,
    },

    /// The resource was already absent.
    ///
    /// This is a successful idempotent outcome.
    AlreadyAbsent,

    /// The resource no longer matches its discovered ownership or revision
    /// evidence and was deliberately left untouched.
    SkippedChanged,
}

// ============================================================================
// PROVIDER CONTRACT
// ============================================================================

/// Authoritative provider for generated-resource discovery and removal.
///
/// Implementations should adapt existing repository authorities rather
/// than create competing artifact or cache systems.
///
/// Examples of valid integrations include:
///
/// - Build-output inventories backed by `BuildOutputs` and the canonical
///   artifact store.
/// - Compiler incremental caches backed by `IncrementalCompiler`.
/// - Backend caches backed by `BackendCache`.
/// - Build-state cleanup backed by `IncrementalBuildCoordinator`.
///
/// A provider must never infer generated ownership from a filename alone.
///
/// # Streaming
///
/// Discovery is visitor-based so the command does not require the provider
/// to materialize every candidate in one unbounded vector.
///
/// Providers must stop discovery when the visitor returns `Stop`.
///
/// # Safe deletion
///
/// `remove` must revalidate the resource identity and ownership evidence
/// immediately before deletion. Filesystem providers must not follow
/// symlinks or traverse outside the canonical project root.
///
/// If the provider cannot guarantee safe removal, it must return an error
/// and leave the resource untouched.
pub trait CleanProvider: Send + Sync {
    /// Visit recognized generated resources in the requested scope.
    fn visit_candidates(
        &self,
        request: &CleanRequest,
        visitor: &mut dyn FnMut(CleanCandidate) -> ToolchainResult<CleanVisitControl>,
    ) -> ToolchainResult<()>;

    /// Remove one previously discovered candidate.
    ///
    /// Implementations must revalidate its identity and revision before
    /// removal. A changed resource must return `SkippedChanged`.
    fn remove(
        &self,
        candidate: &CleanCandidate,
    ) -> ToolchainResult<CleanRemovalOutcome>;
}

// ============================================================================
// CANCELLATION
// ============================================================================

/// Cancellation probe used between clean operations.
pub trait CleanCancellation: Send + Sync {
    /// Return whether the operation has been cancelled.
    fn is_cancelled(&self) -> bool;
}

/// Cancellation implementation that never cancels.
#[derive(Debug, Clone, Copy, Default)]
pub struct NeverCancelled;

impl CleanCancellation for NeverCancelled {
    fn is_cancelled(&self) -> bool {
        false
    }
}

// ============================================================================
// REPORT TYPES
// ============================================================================

/// Final status of a clean operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanStatus {
    /// The operation was a dry run.
    DryRun,

    /// All discovered resources were removed or already absent.
    Complete,

    /// No matching generated resources were discovered.
    NothingToClean,

    /// Some resources were skipped or failed.
    Partial,

    /// The operation was cancelled.
    Cancelled,
}

/// Outcome for one discovered resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanEntryStatus {
    /// The resource would be removed in a non-dry-run operation.
    WouldRemove,

    /// The resource was removed.
    Removed,

    /// The resource was already absent.
    AlreadyAbsent,

    /// The resource changed after discovery and was not removed.
    SkippedChanged,

    /// The provider failed to remove the resource.
    Failed,
}

/// Machine-readable outcome for one resource.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanEntryReport {
    /// Provider identity.
    pub provider_id: String,

    /// Provider-local resource identity.
    pub resource_id: String,

    /// Open-ended resource kind.
    pub kind: String,

    /// Display-only location.
    pub location: String,

    /// Outcome for this resource.
    pub status: CleanEntryStatus,

    /// Actual bytes removed, if known.
    pub bytes_removed: Option<u64>,

    /// Stable provider/toolchain error code, if removal failed.
    pub error_code: Option<String>,

    /// Human-readable failure detail, if removal failed.
    pub error_message: Option<String>,
}

/// Optional discovery-level failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanIssue {
    /// Stable machine-readable error code.
    pub error_code: String,

    /// Human-readable error message.
    pub message: String,
}

/// Structured report for a clean operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanReport {
    /// Report schema identifier.
    pub schema: String,

    /// Final operation status.
    pub status: CleanStatus,

    /// Canonical project or workspace root.
    pub root: String,

    /// Requested resource scope.
    pub scope: CleanScope,

    /// Whether this was a dry run.
    pub dry_run: bool,

    /// Number of unique resources discovered.
    pub discovered: u64,

    /// Number of resources removed.
    pub removed: u64,

    /// Number of resources already absent.
    pub already_absent: u64,

    /// Number of resources skipped because their identity changed.
    pub skipped_changed: u64,

    /// Number of resource-removal failures.
    pub failed: u64,

    /// Aggregate actual bytes removed, if every removal size is known.
    pub bytes_removed: Option<u64>,

    /// Discovery-level error, if discovery failed after partial work.
    pub discovery_issue: Option<CleanIssue>,

    /// Per-resource outcomes.
    pub entries: Vec<CleanEntryReport>,
}

impl CleanReport {
    fn new(request: &CleanRequest) -> Self {
        Self {
            schema: CLEAN_REPORT_SCHEMA.to_owned(),
            status: CleanStatus::NothingToClean,
            root: request.root.display().to_string(),
            scope: request.scope,
            dry_run: request.dry_run,
            discovered: 0,
            removed: 0,
            already_absent: 0,
            skipped_changed: 0,
            failed: 0,
            bytes_removed: Some(0),
            discovery_issue: None,
            entries: Vec::new(),
        }
    }

    /// Return whether every discovered resource was successfully handled.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(
            self.status,
            CleanStatus::Complete | CleanStatus::NothingToClean
        )
    }

    /// Return the number of resources that were removed.
    #[must_use]
    pub const fn removed_count(&self) -> u64 {
        self.removed
    }

    /// Return the number of resources that failed removal.
    #[must_use]
    pub const fn failed_count(&self) -> u64 {
        self.failed
    }
}

// ============================================================================
// COMMAND
// ============================================================================

/// Toolchain clean command backed by an authoritative resource provider.
#[derive(Debug)]
pub struct CleanCommand<P> {
    provider: P,
}

impl<P> CleanCommand<P>
where
    P: CleanProvider,
{
    /// Create a clean command using the supplied provider.
    pub const fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Borrow the provider used by this command.
    #[must_use]
    pub const fn provider(&self) -> &P {
        &self.provider
    }

    /// Execute a clean request without cancellation.
    ///
    /// This operation performs actual removal unless the request is a
    /// dry run.
    pub fn clean(&self, request: CleanRequest) -> ToolchainResult<CleanReport> {
        self.clean_with_cancellation(request, &NeverCancelled)
    }

    /// Execute a clean request with cancellation support.
    ///
    /// Discovery and removal are performed sequentially. This conservative
    /// execution model avoids introducing a hidden worker count or
    /// concurrency policy. A caller that needs parallel cleaning may
    /// implement it inside a provider using its own explicit resource policy.
    pub fn clean_with_cancellation<C>(
        &self,
        request: CleanRequest,
        cancellation: &C,
    ) -> ToolchainResult<CleanReport>
    where
        C: CleanCancellation + ?Sized,
    {
        let request = request.resolve_root()?;
        let mut report = CleanReport::new(&request);

        let mut seen = BTreeSet::<(String, String)>::new();
        let mut cancelled = false;

        let discovery_result = self.provider.visit_candidates(
            &request,
            &mut |candidate| {
                if cancellation.is_cancelled() {
                    cancelled = true;
                    return Ok(CleanVisitControl::Stop);
                }

                validate_candidate_root(&request, &candidate)?;

                let identity = (
                    candidate.provider_id.clone(),
                    candidate.resource_id.clone(),
                );

                if !seen.insert(identity) {
                    return Err(validation_error(format!(
                        "provider returned a duplicate clean candidate: {} / {}",
                        candidate.provider_id,
                        candidate.resource_id
                    )));
                }

                increment(&mut report.discovered, "discovered resource count")?;

                let entry = if request.dry_run {
                    CleanEntryReport {
                        provider_id: candidate.provider_id.clone(),
                        resource_id: candidate.resource_id.clone(),
                        kind: candidate.kind.as_str().to_owned(),
                        location: candidate.location.clone(),
                        status: CleanEntryStatus::WouldRemove,
                        bytes_removed: None,
                        error_code: None,
                        error_message: None,
                    }
                } else {
                    match self.provider.remove(&candidate) {
                        Ok(CleanRemovalOutcome::Removed { bytes_removed }) => {
                            increment(&mut report.removed, "removed resource count")?;
                            add_bytes(&mut report.bytes_removed, bytes_removed)?;

                            CleanEntryReport {
                                provider_id: candidate.provider_id.clone(),
                                resource_id: candidate.resource_id.clone(),
                                kind: candidate.kind.as_str().to_owned(),
                                location: candidate.location.clone(),
                                status: CleanEntryStatus::Removed,
                                bytes_removed,
                                error_code: None,
                                error_message: None,
                            }
                        }

                        Ok(CleanRemovalOutcome::AlreadyAbsent) => {
                            increment(
                                &mut report.already_absent,
                                "already-absent resource count",
                            )?;

                            CleanEntryReport {
                                provider_id: candidate.provider_id.clone(),
                                resource_id: candidate.resource_id.clone(),
                                kind: candidate.kind.as_str().to_owned(),
                                location: candidate.location.clone(),
                                status: CleanEntryStatus::AlreadyAbsent,
                                bytes_removed: Some(0),
                                error_code: None,
                                error_message: None,
                            }
                        }

                        Ok(CleanRemovalOutcome::SkippedChanged) => {
                            increment(
                                &mut report.skipped_changed,
                                "skipped resource count",
                            )?;

                            CleanEntryReport {
                                provider_id: candidate.provider_id.clone(),
                                resource_id: candidate.resource_id.clone(),
                                kind: candidate.kind.as_str().to_owned(),
                                location: candidate.location.clone(),
                                status: CleanEntryStatus::SkippedChanged,
                                bytes_removed: None,
                                error_code: None,
                                error_message: None,
                            }
                        }

                        Err(error) => {
                            increment(&mut report.failed, "failed resource count")?;

                            CleanEntryReport {
                                provider_id: candidate.provider_id.clone(),
                                resource_id: candidate.resource_id.clone(),
                                kind: candidate.kind.as_str().to_owned(),
                                location: candidate.location.clone(),
                                status: CleanEntryStatus::Failed,
                                bytes_removed: None,
                                error_code: Some(error.code().to_owned()),
                                error_message: Some(error.to_string()),
                            }
                        }
                    }
                };

                report
                    .entries
                    .try_reserve(1)
                    .map_err(|error| allocation_error("clean report entries", error))?;

                report.entries.push(entry);

                if cancellation.is_cancelled() {
                    cancelled = true;
                    Ok(CleanVisitControl::Stop)
                } else {
                    Ok(CleanVisitControl::Continue)
                }
            },
        );

        if let Err(error) = discovery_result {
            report.discovery_issue = Some(CleanIssue {
                error_code: error.code().to_owned(),
                message: error.to_string(),
            });
        }

        report.status = determine_status(
            &report,
            cancelled || cancellation.is_cancelled(),
        );

        Ok(report)
    }
}

// ============================================================================
// VALIDATION AND ACCOUNTING
// ============================================================================

fn validate_candidate_root(
    request: &CleanRequest,
    candidate: &CleanCandidate,
) -> ToolchainResult<()> {
    if candidate.root != request.root {
        return Err(validation_error(format!(
            "provider returned a candidate outside the requested root: {}",
            candidate.location
        )));
    }

    if candidate.provider_id.trim().is_empty()
        || candidate.resource_id.trim().is_empty()
        || candidate.revision.trim().is_empty()
    {
        return Err(validation_error(
            "provider returned a candidate with incomplete ownership evidence",
        ));
    }

    Ok(())
}

fn increment(value: &mut u64, subject: &'static str) -> ToolchainResult<()> {
    *value = value.checked_add(1).ok_or_else(|| {
        ToolchainError::resource_exhausted(format!(
            "{subject} exceeded the report representation"
        ))
        .with_operation(OPERATION)
    })?;

    Ok(())
}

fn add_bytes(total: &mut Option<u64>, bytes: Option<u64>) -> ToolchainResult<()> {
    let Some(current) = *total else {
        return Ok(());
    };

    let Some(bytes) = bytes else {
        *total = None;
        return Ok(());
    };

    *total = current.checked_add(bytes);

    if total.is_none() {
        return Err(ToolchainError::resource_exhausted(
            "aggregate removed-byte count overflowed",
        )
        .with_operation(OPERATION));
    }

    Ok(())
}

fn determine_status(report: &CleanReport, cancelled: bool) -> CleanStatus {
    if cancelled {
        return CleanStatus::Cancelled;
    }

    if report.dry_run {
        return CleanStatus::DryRun;
    }

    if report.discovery_issue.is_some()
        || report.failed > 0
        || report.skipped_changed > 0
    {
        return CleanStatus::Partial;
    }

    if report.discovered == 0 {
        return CleanStatus::NothingToClean;
    }

    CleanStatus::Complete
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Mutex;

    static NEXT_ROOT: AtomicU64 = AtomicU64::new(1);

    struct TestRoot(PathBuf);

    impl TestRoot {
        fn new() -> Self {
            let sequence = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "zamani-clean-test-{}-{sequence}",
                std::process::id()
            ));

            std::fs::create_dir_all(&path)
                .expect("test root should be created");

            Self(
                std::fs::canonicalize(&path)
                    .expect("test root should canonicalize"),
            )
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[derive(Debug)]
    struct TestProvider {
        candidates: Vec<CleanCandidate>,
        removed: Mutex<Vec<String>>,
        fail_discovery: bool,
        fail_removal: bool,
        changed: bool,
    }

    impl TestProvider {
        fn new(candidates: Vec<CleanCandidate>) -> Self {
            Self {
                candidates,
                removed: Mutex::new(Vec::new()),
                fail_discovery: false,
                fail_removal: false,
                changed: false,
            }
        }
    }

    impl CleanProvider for TestProvider {
        fn visit_candidates(
            &self,
            _request: &CleanRequest,
            visitor: &mut dyn FnMut(CleanCandidate) -> ToolchainResult<CleanVisitControl>,
        ) -> ToolchainResult<()> {
            if self.fail_discovery {
                return Err(ToolchainError::provider_failed(
                    "test discovery failure",
                )
                .with_operation(OPERATION));
            }

            for candidate in &self.candidates {
                if visitor(candidate.clone())? == CleanVisitControl::Stop {
                    break;
                }
            }

            Ok(())
        }

        fn remove(
            &self,
            candidate: &CleanCandidate,
        ) -> ToolchainResult<CleanRemovalOutcome> {
            if self.fail_removal {
                return Err(ToolchainError::provider_failed(
                    "test removal failure",
                )
                .with_operation(OPERATION));
            }

            if self.changed {
                return Ok(CleanRemovalOutcome::SkippedChanged);
            }

            let mut removed = self
                .removed
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());

            removed.push(candidate.resource_id.clone());

            Ok(CleanRemovalOutcome::Removed {
                bytes_removed: Some(10),
            })
        }
    }

    struct CancelImmediately;

    impl CleanCancellation for CancelImmediately {
        fn is_cancelled(&self) -> bool {
            true
        }
    }

    fn candidate(root: &Path, id: &str) -> CleanCandidate {
        CleanCandidate::new(
            "test-provider",
            id,
            CleanResourceKind::new("build.output")
                .expect("kind should be valid"),
            root.to_path_buf(),
            format!("generated resource {id}"),
            format!("revision-{id}"),
            Some(10),
        )
        .expect("candidate should be valid")
    }

    #[test]
    fn request_rejects_empty_root() {
        assert!(CleanRequest::new(PathBuf::new()).is_err());
    }

    #[test]
    fn resource_kind_is_open_ended() {
        let kind = CleanResourceKind::new("future.domain.generated-resource")
            .expect("open-ended resource kinds should be accepted");

        assert_eq!(kind.as_str(), "future.domain.generated-resource");
    }

    #[test]
    fn resource_kind_rejects_empty_identity() {
        assert!(CleanResourceKind::new("  ").is_err());
    }

    #[test]
    fn candidate_rejects_relative_root() {
        let result = CleanCandidate::new(
            "provider",
            "resource",
            CleanResourceKind::new("cache").expect("kind"),
            "relative/root",
            "cache entry",
            "revision",
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn dry_run_discovers_without_removing() {
        let root = TestRoot::new();
        let provider = TestProvider::new(vec![candidate(root.path(), "one")]);
        let command = CleanCommand::new(provider);

        let request = CleanRequest::new(root.path())
            .expect("request")
            .with_dry_run(true);

        let report = command.clean(request).expect("dry run");

        assert_eq!(report.status, CleanStatus::DryRun);
        assert_eq!(report.discovered, 1);
        assert_eq!(report.removed, 0);
        assert_eq!(report.entries[0].status, CleanEntryStatus::WouldRemove);
        assert!(command.provider().removed.lock().unwrap().is_empty());
    }

    #[test]
    fn removes_recognized_resources_and_reports_bytes() {
        let root = TestRoot::new();

        let provider = TestProvider::new(vec![
            candidate(root.path(), "one"),
            candidate(root.path(), "two"),
        ]);

        let command = CleanCommand::new(provider);
        let request = CleanRequest::new(root.path()).expect("request");

        let report = command.clean(request).expect("clean");

        assert_eq!(report.status, CleanStatus::Complete);
        assert_eq!(report.discovered, 2);
        assert_eq!(report.removed, 2);
        assert_eq!(report.bytes_removed, Some(20));
    }

    #[test]
    fn changed_resource_is_not_removed() {
        let root = TestRoot::new();
        let mut provider = TestProvider::new(vec![candidate(root.path(), "one")]);
        provider.changed = true;

        let command = CleanCommand::new(provider);
        let request = CleanRequest::new(root.path()).expect("request");

        let report = command.clean(request).expect("clean");

        assert_eq!(report.status, CleanStatus::Partial);
        assert_eq!(report.skipped_changed, 1);
        assert_eq!(report.removed, 0);
    }

    #[test]
    fn provider_removal_failure_is_reported() {
        let root = TestRoot::new();
        let mut provider = TestProvider::new(vec![candidate(root.path(), "one")]);
        provider.fail_removal = true;

        let command = CleanCommand::new(provider);
        let request = CleanRequest::new(root.path()).expect("request");

        let report = command.clean(request).expect("report");

        assert_eq!(report.status, CleanStatus::Partial);
        assert_eq!(report.failed, 1);
        assert_eq!(report.entries[0].status, CleanEntryStatus::Failed);
        assert!(report.entries[0].error_code.is_some());
    }

    #[test]
    fn discovery_failure_is_not_reported_as_success() {
        let root = TestRoot::new();
        let mut provider = TestProvider::new(Vec::new());
        provider.fail_discovery = true;

        let command = CleanCommand::new(provider);
        let request = CleanRequest::new(root.path()).expect("request");

        let report = command.clean(request).expect("report");

        assert_eq!(report.status, CleanStatus::Partial);
        assert!(report.discovery_issue.is_some());
    }

    #[test]
    fn cancellation_stops_before_removal() {
        let root = TestRoot::new();
        let provider = TestProvider::new(vec![candidate(root.path(), "one")]);
        let command = CleanCommand::new(provider);

        let request = CleanRequest::new(root.path()).expect("request");

        let report = command
            .clean_with_cancellation(request, &CancelImmediately)
            .expect("cancelled report");

        assert_eq!(report.status, CleanStatus::Cancelled);
        assert_eq!(report.removed, 0);
    }

    #[test]
    fn empty_inventory_is_reported_explicitly() {
        let root = TestRoot::new();
        let provider = TestProvider::new(Vec::new());
        let command = CleanCommand::new(provider);

        let request = CleanRequest::new(root.path()).expect("request");
        let report = command.clean(request).expect("clean");

        assert_eq!(report.status, CleanStatus::NothingToClean);
        assert_eq!(report.discovered, 0);
    }

    #[test]
    fn report_serialization_uses_stable_schema_and_status() {
        let root = TestRoot::new();
        let provider = TestProvider::new(Vec::new());
        let command = CleanCommand::new(provider);

        let request = CleanRequest::new(root.path())
            .expect("request")
            .with_dry_run(true);

        let report = command.clean(request).expect("clean");
        let json = serde_json::to_string(&report).expect("serialize");

        assert!(json.contains(CLEAN_REPORT_SCHEMA));
        assert!(json.contains("\"status\":\"dry_run\""));
        assert!(json.contains("\"discovered\":0"));
    }

    #[test]
    fn unknown_removed_byte_count_is_not_reported_as_zero() {
        let mut total = Some(0);

        add_bytes(&mut total, Some(10)).expect("first size");
        add_bytes(&mut total, None).expect("unknown size");
        add_bytes(&mut total, Some(20)).expect("later size");

        assert_eq!(total, None);
    }

    #[test]
    fn report_count_overflow_is_reported() {
        let mut count = u64::MAX;

        assert!(increment(&mut count, "test count").is_err());
    }
}
