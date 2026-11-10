
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani compiler extension discovery.
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Discover declarative compiler extension manifests and convert
//! structurally valid manifests into provider candidates.
//!
//! Discovery does not register, initialize, load, authorize or execute
//! extension providers.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Discovery requests.
//! - Discovery roots.
//! - Filesystem traversal.
//! - Manifest filename recognition.
//! - Manifest parsing dispatch.
//! - Discovery budgets.
//! - Discovery issues and reports.
//! - Deterministic candidate ordering.
//! - Duplicate provider identity detection.
//!
//! This module does not own:
//!
//! - Provider identity or provider version definitions.
//! - Manifest schema definitions.
//! - Provider compatibility evaluation.
//! - Provider registration.
//! - Extension admission.
//! - Supply-chain verification.
//! - Dynamic library loading.
//! - Provider initialization.
//! - Extension isolation.
//! - Backend registration.
//! - Pass registration.
//! - Target discovery.
//! - Hardware discovery.
//! - Resource allocation.
//! - Scheduling.
//! - Runtime execution.
//!
//! # Canonical authorities
//!
//! Provider contracts:
//!     crate::compiler::extensions::provider
//!
//! Manifest parsing and structural validation:
//!     crate::compiler::extensions::manifest
//!
//! Provider compatibility:
//!     crate::compiler::extensions::compatibility
//!
//! Supply-chain verification:
//!     crate::compiler::security::supply_chain
//!
//! Backend registration:
//!     crate::backend::registry
//!
//! Compiler backend integration:
//!     crate::compiler::backends
//!
//! Compiler pass registration:
//!     crate::compiler::passes
//!
//! Target discovery:
//!     crate::compiler::planning::target_discovery
//!
//! # Security
//!
//! Manifest files are untrusted input.
//!
//! This module:
//!
//! - Never executes manifest contents.
//! - Never loads provider binaries.
//! - Never follows filesystem symlinks.
//! - Supports caller-supplied manifest input budgets.
//! - Reports incomplete traversal explicitly.
//! - Does not treat successful parsing as proof of trust.
//!
//! A discovered provider is only a candidate.
//!
//! # Scalability
//!
//! No arbitrary universal provider-count, extension-count,
//! directory-count or computing-domain-count limit is imposed.
//!
//! Optional operational budgets are supplied by callers.
//!
//! An absent budget means that this module does not impose a limit
//! for that dimension.
//!
//! Actual processing remains constrained by available memory,
//! address space, operating-system limits and configured policies.
//!
//! Literal infinite filesystem traversal is not promised.
//!
//! # Determinism
//!
//! Roots, directory entries, candidates and issues are sorted.
//!
//! Filesystem enumeration order does not determine provider identity
//! or candidate precedence.
//!
//! Duplicate provider identities are reported rather than resolved
//! by silently choosing the first discovered candidate.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use super::manifest::{
    ExtensionManifest,
    ManifestError,
    ManifestInputBudget,
    MANIFEST_FILE_NAME,
};
use super::provider::{
    ProviderDescriptor,
    ProviderId,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable discovery module identity.
pub const DISCOVERY_MODULE_NAME: &str =
    "zamani.compiler.extensions.discovery";

/// JSON extension manifest filename.
///
/// TOML remains the canonical default manifest format.
pub const JSON_MANIFEST_FILE_NAME: &str =
    "zamani-extension.json";

// ============================================================================
// DISCOVERY REQUEST
// ============================================================================

/// Request to discover extension manifests.
///
/// Roots are directories containing extension packages or directories
/// containing nested extension packages.
///
/// Root order does not determine discovery result order.
///
/// A request must contain at least one root.
#[derive(Clone, Debug)]
pub struct DiscoveryRequest {
    roots: Vec<PathBuf>,
    options: DiscoveryOptions,
}

impl DiscoveryRequest {
    /// Creates a discovery request.
    ///
    /// The roots are converted to owned paths.
    ///
    /// An empty root collection is rejected when discovery executes.
    #[must_use]
    pub fn new(
        roots: impl IntoIterator<Item = PathBuf>,
    ) -> Self {
        Self {
            roots: roots.into_iter().collect(),
            options: DiscoveryOptions::default(),
        }
    }

    /// Replaces the discovery options.
    #[must_use]
    pub fn with_options(
        mut self,
        options: DiscoveryOptions,
    ) -> Self {
        self.options = options;
        self
    }

    /// Returns the requested roots.
    #[must_use]
    pub fn roots(&self) -> &[PathBuf] {
        &self.roots
    }

    /// Returns the discovery options.
    #[must_use]
    pub const fn options(&self) -> &DiscoveryOptions {
        &self.options
    }
}

// ============================================================================
// DISCOVERY OPTIONS
// ============================================================================

/// Operational options controlling filesystem discovery.
///
/// These options do not change provider semantics or manifest meaning.
///
/// An absent budget means no additional limit is imposed by this
/// discovery implementation for that dimension.
#[derive(Clone, Debug)]
pub struct DiscoveryOptions {
    recursive: bool,
    maximum_depth: Option<usize>,
    maximum_entries: Option<usize>,
    manifest_budget: ManifestInputBudget,
}

impl Default for DiscoveryOptions {
    fn default() -> Self {
        Self {
            recursive: true,
            maximum_depth: None,
            maximum_entries: None,
            manifest_budget: ManifestInputBudget::unlimited(),
        }
    }
}

impl DiscoveryOptions {
    /// Creates default discovery options.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables or disables recursive directory traversal.
    ///
    /// When disabled, only the direct contents of each root are
    /// inspected.
    #[must_use]
    pub const fn recursive(
        mut self,
        recursive: bool,
    ) -> Self {
        self.recursive = recursive;
        self
    }

    /// Sets the maximum directory depth.
    ///
    /// The root itself has depth zero.
    ///
    /// If traversal encounters a directory beyond this depth,
    /// discovery is reported as incomplete.
    ///
    /// `None` means no additional depth limit.
    #[must_use]
    pub const fn maximum_depth(
        mut self,
        maximum_depth: Option<usize>,
    ) -> Self {
        self.maximum_depth = maximum_depth;
        self
    }

    /// Sets the maximum number of filesystem entries visited.
    ///
    /// Entries include files and directories.
    ///
    /// If further entries remain when the budget is exhausted,
    /// discovery is reported as incomplete.
    ///
    /// `None` means no additional entry limit.
    #[must_use]
    pub const fn maximum_entries(
        mut self,
        maximum_entries: Option<usize>,
    ) -> Self {
        self.maximum_entries = maximum_entries;
        self
    }

    /// Sets the operational budget for parsing manifests.
    ///
    /// This budget is delegated to the canonical manifest parser.
    #[must_use]
    pub fn manifest_budget(
        mut self,
        budget: ManifestInputBudget,
    ) -> Self {
        self.manifest_budget = budget;
        self
    }

    /// Returns whether recursive traversal is enabled.
    #[must_use]
    pub const fn is_recursive(&self) -> bool {
        self.recursive
    }

    /// Returns the optional maximum depth.
    #[must_use]
    pub const fn maximum_depth_limit(
        &self,
    ) -> Option<usize> {
        self.maximum_depth
    }

    /// Returns the optional maximum entry count.
    #[must_use]
    pub const fn maximum_entries_limit(
        &self,
    ) -> Option<usize> {
        self.maximum_entries
    }

    /// Returns the manifest input budget.
    #[must_use]
    pub const fn manifest_input_budget(
        &self,
    ) -> &ManifestInputBudget {
        &self.manifest_budget
    }
}

// ============================================================================
// MANIFEST FORMAT
// ============================================================================

/// Recognized extension manifest serialization format.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
)]
pub enum ManifestFormat {
    /// TOML extension manifest.
    Toml,

    /// JSON extension manifest.
    Json,
}

impl ManifestFormat {
    /// Detects a manifest format from its exact filename.
    ///
    /// Unrecognized filenames are not treated as manifests.
    #[must_use]
    pub fn from_path(path: &Path) -> Option<Self> {
        match path.file_name()?.to_str()? {
            MANIFEST_FILE_NAME => Some(Self::Toml),
            JSON_MANIFEST_FILE_NAME => Some(Self::Json),
            _ => None,
        }
    }

    /// Returns the canonical filename for this format.
    #[must_use]
    pub const fn filename(self) -> &'static str {
        match self {
            Self::Toml => MANIFEST_FILE_NAME,
            Self::Json => JSON_MANIFEST_FILE_NAME,
        }
    }
}

// ============================================================================
// DISCOVERY CANDIDATE
// ============================================================================

/// A structurally valid provider candidate discovered on disk.
///
/// A candidate contains declarative metadata only.
///
/// It is not registered, trusted, compatible or initialized merely
/// because discovery returned it.
#[derive(Clone, Debug)]
pub struct DiscoveryCandidate {
    root: PathBuf,
    manifest_path: PathBuf,
    format: ManifestFormat,
    manifest: ExtensionManifest,
    descriptor: ProviderDescriptor,
}

impl DiscoveryCandidate {
    /// Returns the canonical discovery root containing this candidate.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Returns the canonical manifest path.
    #[must_use]
    pub fn manifest_path(&self) -> &Path {
        &self.manifest_path
    }

    /// Returns the manifest format.
    #[must_use]
    pub const fn format(&self) -> ManifestFormat {
        self.format
    }

    /// Returns the parsed manifest.
    #[must_use]
    pub fn manifest(&self) -> &ExtensionManifest {
        &self.manifest
    }

    /// Returns the canonical provider descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ProviderDescriptor {
        &self.descriptor
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        self.descriptor.id()
    }
}

// ============================================================================
// DISCOVERY ISSUE
// ============================================================================

/// Machine-readable category of a discovery issue.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
)]
#[non_exhaustive]
pub enum DiscoveryIssueKind {
    /// A requested root could not be inspected.
    RootUnavailable,

    /// A requested root is not a directory.
    RootNotDirectory,

    /// A requested root is a symbolic link.
    SymbolicLinkRoot,

    /// A directory could not be read.
    DirectoryReadFailed,

    /// A directory entry could not be inspected.
    EntryReadFailed,

    /// A symbolic link was intentionally not followed.
    SymbolicLinkSkipped,

    /// A manifest file could not be opened or read.
    ManifestReadFailed,

    /// A manifest exceeded its configured input budget.
    ManifestBudgetExceeded,

    /// A manifest could not be parsed.
    ManifestParseFailed,

    /// A parsed manifest failed structural validation.
    ManifestInvalid,

    /// Multiple manifests declare the same provider identity.
    DuplicateProviderIdentity,

    /// The directory depth budget prevented further traversal.
    DepthBudgetExceeded,

    /// The filesystem entry budget prevented further traversal.
    EntryBudgetExceeded,

    /// A path could not be canonicalized.
    PathResolutionFailed,
}

/// One structured issue encountered during discovery.
///
/// Issues are data, not executable instructions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryIssue {
    kind: DiscoveryIssueKind,
    path: PathBuf,
    message: String,
}

impl DiscoveryIssue {
    fn new(
        kind: DiscoveryIssueKind,
        path: impl Into<PathBuf>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            path: path.into(),
            message: message.into(),
        }
    }

    /// Returns the issue category.
    #[must_use]
    pub const fn kind(&self) -> DiscoveryIssueKind {
        self.kind
    }

    /// Returns the path associated with the issue.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the human-readable explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// DUPLICATE PROVIDER IDENTITIES
// ============================================================================

/// A provider identity declared by more than one discovered manifest.
///
/// Discovery never resolves this conflict by selecting an arbitrary
/// candidate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderIdentityConflict {
    provider_id: ProviderId,
    manifest_paths: Vec<PathBuf>,
}

impl ProviderIdentityConflict {
    /// Returns the conflicting provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns all manifest paths declaring the identity.
    #[must_use]
    pub fn manifest_paths(&self) -> &[PathBuf] {
        &self.manifest_paths
    }
}

// ============================================================================
// DISCOVERY REPORT
// ============================================================================

/// Result of one extension discovery operation.
///
/// A report may contain valid candidates and issues simultaneously.
///
/// `is_complete()` describes whether the requested traversal scope
/// was completely inspected. It does not mean that every manifest
/// was valid or that every provider is compatible or trusted.
#[derive(Clone, Debug)]
pub struct DiscoveryReport {
    roots: Vec<PathBuf>,
    candidates: Vec<DiscoveryCandidate>,
    issues: Vec<DiscoveryIssue>,
    conflicts: Vec<ProviderIdentityConflict>,
    entries_visited: usize,
    complete: bool,
}

impl DiscoveryReport {
    /// Returns canonical roots successfully accepted for traversal.
    #[must_use]
    pub fn roots(&self) -> &[PathBuf] {
        &self.roots
    }

    /// Returns structurally valid provider candidates.
    ///
    /// Candidates are ordered by provider identity and then by
    /// canonical manifest path.
    #[must_use]
    pub fn candidates(&self) -> &[DiscoveryCandidate] {
        &self.candidates
    }

    /// Returns all discovery issues.
    ///
    /// Issues are returned in deterministic order.
    #[must_use]
    pub fn issues(&self) -> &[DiscoveryIssue] {
        &self.issues
    }

    /// Returns duplicate provider identity conflicts.
    #[must_use]
    pub fn conflicts(&self) -> &[ProviderIdentityConflict] {
        &self.conflicts
    }

    /// Returns the number of filesystem entries visited.
    #[must_use]
    pub const fn entries_visited(&self) -> usize {
        self.entries_visited
    }

    /// Returns whether the requested traversal scope was completed.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }

    /// Returns whether the report contains no discovery issues.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.issues.is_empty()
            && self.conflicts.is_empty()
            && self.complete
    }

    /// Returns whether any provider identity conflict exists.
    #[must_use]
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }

    /// Returns candidates that do not have a duplicate identity.
    ///
    /// This is a convenience view only. It does not perform provider
    /// compatibility evaluation or security admission.
    #[must_use]
    pub fn unambiguous_candidates(
        &self,
    ) -> Vec<&DiscoveryCandidate> {
        let conflicted: BTreeSet<&ProviderId> = self
            .conflicts
            .iter()
            .map(ProviderIdentityConflict::provider_id)
            .collect();

        self.candidates
            .iter()
            .filter(|candidate| {
                !conflicted.contains(candidate.provider_id())
            })
            .collect()
    }
}

// ============================================================================
// DISCOVERY ERRORS
// ============================================================================

/// Errors that prevent a discovery request from being executed.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DiscoveryError {
    /// No discovery roots were supplied.
    NoRoots,

    /// An internal entry counter exceeded its representable range.
    CounterOverflow,
}

impl fmt::Display for DiscoveryError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::NoRoots => {
                formatter.write_str(
                    "extension discovery requires at least one root",
                )
            }
            Self::CounterOverflow => {
                formatter.write_str(
                    "extension discovery counter overflowed",
                )
            }
        }
    }
}

impl Error for DiscoveryError {}

// ============================================================================
// DISCOVERY SERVICE
// ============================================================================

/// Stateless extension discovery service.
///
/// The service can be reused across compilation sessions.
///
/// It does not own a provider registry.
#[derive(Clone, Copy, Debug, Default)]
pub struct ExtensionDiscovery;

impl ExtensionDiscovery {
    /// Creates a discovery service.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Discovers extension manifests for the supplied request.
    ///
    /// Filesystem and manifest problems are returned as report issues.
    ///
    /// Invalid request structure and counter overflow are returned
    /// as `DiscoveryError`.
    pub fn discover(
        &self,
        request: &DiscoveryRequest,
    ) -> Result<DiscoveryReport, DiscoveryError> {
        if request.roots.is_empty() {
            return Err(DiscoveryError::NoRoots);
        }

        let mut report = DiscoveryReport {
            roots: Vec::new(),
            candidates: Vec::new(),
            issues: Vec::new(),
            conflicts: Vec::new(),
            entries_visited: 0,
            complete: true,
        };

        let mut canonical_roots = BTreeSet::new();

        for requested_root in &request.roots {
            match prepare_root(requested_root) {
                Ok(root) => {
                    if canonical_roots.insert(root.clone()) {
                        report.roots.push(root);
                    }
                }
                Err(issue) => {
                    report.complete = false;
                    report.issues.push(issue);
                }
            }
        }

        report.roots.sort();

        let mut queue = VecDeque::new();

        for root in &report.roots {
            queue.push_back((root.clone(), 0usize, root.clone()));
        }

        while let Some((directory, depth, root)) = queue.pop_front() {
            self.scan_directory(
                &directory,
                depth,
                &root,
                request.options(),
                &mut queue,
                &mut report,
            )?;
        }

        self.detect_duplicate_identities(&mut report);

        report.candidates.sort_by(|left, right| {
            left.provider_id()
                .cmp(right.provider_id())
                .then_with(|| {
                    left.manifest_path()
                        .cmp(right.manifest_path())
                })
        });

        report.issues.sort_by(|left, right| {
            left.path
                .cmp(&right.path)
                .then_with(|| left.kind.cmp(&right.kind))
                .then_with(|| left.message.cmp(&right.message))
        });

        report.conflicts.sort_by(|left, right| {
            left.provider_id.cmp(&right.provider_id)
        });

        Ok(report)
    }

    fn scan_directory(
        &self,
        directory: &Path,
        depth: usize,
        root: &Path,
        options: &DiscoveryOptions,
        queue: &mut VecDeque<(PathBuf, usize, PathBuf)>,
        report: &mut DiscoveryReport,
    ) -> Result<(), DiscoveryError> {
        let read_dir = match fs::read_dir(directory) {
            Ok(entries) => entries,
            Err(error) => {
                report.complete = false;
                report.issues.push(DiscoveryIssue::new(
                    DiscoveryIssueKind::DirectoryReadFailed,
                    directory,
                    error.to_string(),
                ));
                return Ok(());
            }
        };

        let mut entries = Vec::new();

        for entry_result in read_dir {
            if let Some(maximum) = options.maximum_entries {
                if report.entries_visited >= maximum {
                    report.complete = false;
                    report.issues.push(DiscoveryIssue::new(
                        DiscoveryIssueKind::EntryBudgetExceeded,
                        directory,
                        format!(
                            "filesystem entry budget of {maximum} \
                             was exhausted"
                        ),
                    ));
                    break;
                }
            }

            report.entries_visited = report
                .entries_visited
                .checked_add(1)
                .ok_or(DiscoveryError::CounterOverflow)?;

            match entry_result {
                Ok(entry) => entries.push(entry),
                Err(error) => {
                    report.complete = false;
                    report.issues.push(DiscoveryIssue::new(
                        DiscoveryIssueKind::EntryReadFailed,
                        directory,
                        error.to_string(),
                    ));
                }
            }
        }

        entries.sort_by_key(|entry| entry.file_name());

        for entry in entries {
            let path = entry.path();

            let file_type = match entry.file_type() {
                Ok(file_type) => file_type,
                Err(error) => {
                    report.complete = false;
                    report.issues.push(DiscoveryIssue::new(
                        DiscoveryIssueKind::EntryReadFailed,
                        path,
                        error.to_string(),
                    ));
                    continue;
                }
            };

            if file_type.is_symlink() {
                report.issues.push(DiscoveryIssue::new(
                    DiscoveryIssueKind::SymbolicLinkSkipped,
                    path,
                    "symbolic links are not followed during discovery",
                ));
                continue;
            }

            if file_type.is_dir() {
                if !options.recursive {
                    continue;
                }

                let next_depth = match depth.checked_add(1) {
                    Some(value) => value,
                    None => {
                        report.complete = false;
                        report.issues.push(DiscoveryIssue::new(
                            DiscoveryIssueKind::DepthBudgetExceeded,
                            path,
                            "directory depth counter overflowed",
                        ));
                        continue;
                    }
                };

                if options
                    .maximum_depth
                    .is_some_and(|maximum| next_depth > maximum)
                {
                    report.complete = false;
                    report.issues.push(DiscoveryIssue::new(
                        DiscoveryIssueKind::DepthBudgetExceeded,
                        path,
                        format!(
                            "directory depth budget of {} was reached",
                            options.maximum_depth.unwrap_or_default()
                        ),
                    ));
                    continue;
                }

                queue.push_back((path, next_depth, root.to_path_buf()));
                continue;
            }

            if !file_type.is_file() {
                continue;
            }

            let Some(format) = ManifestFormat::from_path(&path) else {
                continue;
            };

            self.read_candidate(
                &path,
                root,
                format,
                options,
                report,
            );
        }

        Ok(())
    }

    fn read_candidate(
        &self,
        path: &Path,
        root: &Path,
        format: ManifestFormat,
        options: &DiscoveryOptions,
        report: &mut DiscoveryReport,
    ) {
        let input = match read_manifest_input(
            path,
            options.manifest_input_budget(),
        ) {
            Ok(input) => input,
            Err(error) => {
                let kind = match error {
                    ManifestReadError::BudgetExceeded { .. } => {
                        DiscoveryIssueKind::ManifestBudgetExceeded
                    }
                    ManifestReadError::Io(_) => {
                        DiscoveryIssueKind::ManifestReadFailed
                    }
                };

                report.issues.push(DiscoveryIssue::new(
                    kind,
                    path,
                    error.to_string(),
                ));
                return;
            }
        };

        let manifest_result = match format {
            ManifestFormat::Toml => {
                ExtensionManifest::from_toml_with_budget(
                    &input,
                    options.manifest_input_budget(),
                )
            }
            ManifestFormat::Json => {
                ExtensionManifest::from_json_with_budget(
                    &input,
                    options.manifest_input_budget(),
                )
            }
        };

        let manifest = match manifest_result {
            Ok(manifest) => manifest,
            Err(error) => {
                report.issues.push(DiscoveryIssue::new(
                    DiscoveryIssueKind::ManifestParseFailed,
                    path,
                    error.to_string(),
                ));
                return;
            }
        };

        let descriptor = match manifest.to_provider_descriptor() {
            Ok(descriptor) => descriptor,
            Err(error) => {
                report.issues.push(DiscoveryIssue::new(
                    DiscoveryIssueKind::ManifestInvalid,
                    path,
                    error.to_string(),
                ));
                return;
            }
        };

        let canonical_manifest_path = match fs::canonicalize(path) {
            Ok(path) => path,
            Err(error) => {
                report.complete = false;
                report.issues.push(DiscoveryIssue::new(
                    DiscoveryIssueKind::PathResolutionFailed,
                    path,
                    error.to_string(),
                ));
                return;
            }
        };

        report.candidates.push(DiscoveryCandidate {
            root: root.to_path_buf(),
            manifest_path: canonical_manifest_path,
            format,
            manifest,
            descriptor,
        });
    }

    fn detect_duplicate_identities(
        &self,
        report: &mut DiscoveryReport,
    ) {
        let mut identities: BTreeMap<ProviderId, Vec<PathBuf>> =
            BTreeMap::new();

        for candidate in &report.candidates {
            identities
                .entry(candidate.provider_id().clone())
                .or_default()
                .push(candidate.manifest_path().to_path_buf());
        }

        for (provider_id, mut manifest_paths) in identities {
            if manifest_paths.len() < 2 {
                continue;
            }

            manifest_paths.sort();

            let description = manifest_paths
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ");

            report.issues.push(DiscoveryIssue::new(
                DiscoveryIssueKind::DuplicateProviderIdentity,
                manifest_paths[0].clone(),
                format!(
                    "provider identity `{provider_id}` is declared \
                     by multiple manifests: {description}"
                ),
            ));

            report.conflicts.push(ProviderIdentityConflict {
                provider_id,
                manifest_paths,
            });
        }
    }
}

// ============================================================================
// PUBLIC CONVENIENCE FUNCTION
// ============================================================================

/// Discovers extension manifests using the supplied request.
///
/// This is equivalent to calling `ExtensionDiscovery::new().discover`.
pub fn discover(
    request: &DiscoveryRequest,
) -> Result<DiscoveryReport, DiscoveryError> {
    ExtensionDiscovery::new().discover(request)
}

// ============================================================================
// ROOT VALIDATION
// ============================================================================

fn prepare_root(
    requested_root: &Path,
) -> Result<PathBuf, DiscoveryIssue> {
    let metadata = fs::symlink_metadata(requested_root).map_err(
        |error| {
            DiscoveryIssue::new(
                DiscoveryIssueKind::RootUnavailable,
                requested_root,
                error.to_string(),
            )
        },
    )?;

    if metadata.file_type().is_symlink() {
        return Err(DiscoveryIssue::new(
            DiscoveryIssueKind::SymbolicLinkRoot,
            requested_root,
            "symbolic-link discovery roots are not followed",
        ));
    }

    if !metadata.is_dir() {
        return Err(DiscoveryIssue::new(
            DiscoveryIssueKind::RootNotDirectory,
            requested_root,
            "discovery root is not a directory",
        ));
    }

    fs::canonicalize(requested_root).map_err(|error| {
        DiscoveryIssue::new(
            DiscoveryIssueKind::PathResolutionFailed,
            requested_root,
            error.to_string(),
        )
    })
}

// ============================================================================
// BOUNDED MANIFEST INPUT
// ============================================================================

enum ManifestReadError {
    Io(io::Error),

    BudgetExceeded {
        maximum: usize,
        actual_at_least: u64,
    },
}

impl fmt::Display for ManifestReadError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "{error}"),
            Self::BudgetExceeded {
                maximum,
                actual_at_least,
            } => write!(
                formatter,
                "manifest input exceeds the configured limit \
                 of {maximum} bytes (read at least {actual_at_least})"
            ),
        }
    }
}

fn read_manifest_input(
    path: &Path,
    budget: &ManifestInputBudget,
) -> Result<String, ManifestReadError> {
    let file = File::open(path).map_err(ManifestReadError::Io)?;

    let mut input = String::new();

    match budget.maximum_input_bytes {
        None => {
            let mut file = file;
            file.read_to_string(&mut input)
                .map_err(ManifestReadError::Io)?;
        }
        Some(maximum) => {
            let limit = u64::try_from(maximum)
                .unwrap_or(u64::MAX)
                .saturating_add(1);

            let mut bounded = file.take(limit);

            bounded
                .read_to_string(&mut input)
                .map_err(ManifestReadError::Io)?;

            if input.len() > maximum {
                return Err(ManifestReadError::BudgetExceeded {
                    maximum,
                    actual_at_least: input.len() as u64,
                });
            }
        }
    }

    Ok(input)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicU64, Ordering};

    use super::super::manifest::{
        ExtensionManifest,
        ManifestApiCompatibility,
    };

    static NEXT_DIRECTORY: AtomicU64 =
        AtomicU64::new(0);

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new() -> Self {
            let sequence = NEXT_DIRECTORY.fetch_add(
                1,
                Ordering::Relaxed,
            );

            let path = std::env::temp_dir().join(format!(
                "zamani-extension-discovery-{}-{sequence}",
                std::process::id(),
            ));

            fs::create_dir_all(&path)
                .expect("test directory should be created");

            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn write_manifest(
        directory: &Path,
        provider_id: &str,
    ) -> PathBuf {
        fs::create_dir_all(directory)
            .expect("manifest directory should be created");

        let manifest = ExtensionManifest::new(
            provider_id,
            "1.0.0",
            ManifestApiCompatibility {
                major: 1,
                minimum_minor: 0,
                maximum_minor: None,
            },
            "Test extension",
            vec!["backend".to_owned()],
        );

        let contents = manifest
            .to_toml()
            .expect("test manifest should serialize");

        let path = directory.join(MANIFEST_FILE_NAME);

        fs::write(&path, contents)
            .expect("manifest should be written");

        path
    }

    #[test]
    fn discovers_a_manifest_in_the_root() {
        let directory = TestDirectory::new();

        write_manifest(
            directory.path(),
            "org.example.root",
        );

        let request = DiscoveryRequest::new([
            directory.path().to_path_buf(),
        ]);

        let report = discover(&request)
            .expect("discovery should execute");

        assert_eq!(report.candidates().len(), 1);
        assert!(report.is_complete());
        assert_eq!(
            report.candidates()[0]
                .provider_id()
                .as_str(),
            "org.example.root",
        );
    }

    #[test]
    fn recursively_discovers_nested_manifests() {
        let directory = TestDirectory::new();

        write_manifest(
            &directory.path().join("first"),
            "org.example.first",
        );

        write_manifest(
            &directory.path().join("nested").join("second"),
            "org.example.second",
        );

        let request = DiscoveryRequest::new([
            directory.path().to_path_buf(),
        ]);

        let report = discover(&request)
            .expect("discovery should execute");

        assert_eq!(report.candidates().len(), 2);

        assert_eq!(
            report.candidates()[0]
                .provider_id()
                .as_str(),
            "org.example.first",
        );

        assert_eq!(
            report.candidates()[1]
                .provider_id()
                .as_str(),
            "org.example.second",
        );
    }

    #[test]
    fn non_recursive_discovery_does_not_visit_nested_directories() {
        let directory = TestDirectory::new();

        write_manifest(
            &directory.path().join("nested"),
            "org.example.nested",
        );

        let request = DiscoveryRequest::new([
            directory.path().to_path_buf(),
        ])
        .with_options(
            DiscoveryOptions::new().recursive(false),
        );

        let report = discover(&request)
            .expect("discovery should execute");

        assert!(report.candidates().is_empty());
        assert!(report.is_complete());
    }

    #[test]
    fn reports_duplicate_provider_identities() {
        let directory = TestDirectory::new();

        write_manifest(
            &directory.path().join("first"),
            "org.example.duplicate",
        );

        write_manifest(
            &directory.path().join("second"),
            "org.example.duplicate",
        );

        let request = DiscoveryRequest::new([
            directory.path().to_path_buf(),
        ]);

        let report = discover(&request)
            .expect("discovery should execute");

        assert_eq!(report.candidates().len(), 2);
        assert_eq!(report.conflicts().len(), 1);
        assert_eq!(
            report.conflicts()[0].manifest_paths().len(),
            2,
        );
        assert!(report.has_conflicts());
        assert!(report.unambiguous_candidates().is_empty());
    }

    #[test]
    fn reports_invalid_manifest_without_panicking() {
        let directory = TestDirectory::new();

        fs::write(
            directory.path().join(MANIFEST_FILE_NAME),
            "this is not valid TOML = =",
        )
        .expect("invalid manifest should be written");

        let request = DiscoveryRequest::new([
            directory.path().to_path_buf(),
        ]);

        let report = discover(&request)
            .expect("discovery should return a report");

        assert!(report.candidates().is_empty());
        assert_eq!(report.issues().len(), 1);
        assert_eq!(
            report.issues()[0].kind(),
            DiscoveryIssueKind::ManifestParseFailed,
        );
    }

    #[test]
    fn reports_depth_budget_exhaustion() {
        let directory = TestDirectory::new();

        write_manifest(
            &directory.path().join("nested"),
            "org.example.depth",
        );

        let request = DiscoveryRequest::new([
            directory.path().to_path_buf(),
        ])
        .with_options(
            DiscoveryOptions::new().maximum_depth(Some(0)),
        );

        let report = discover(&request)
            .expect("discovery should execute");

        assert!(report.candidates().is_empty());
        assert!(!report.is_complete());
        assert!(report.issues().iter().any(|issue| {
            issue.kind() == DiscoveryIssueKind::DepthBudgetExceeded
        }));
    }

    #[test]
    fn reports_entry_budget_exhaustion() {
        let directory = TestDirectory::new();

        write_manifest(
            &directory.path().join("first"),
            "org.example.first",
        );

        fs::create_dir_all(directory.path().join("another"))
            .expect("nested directory should be created");

        let request = DiscoveryRequest::new([
            directory.path().to_path_buf(),
        ])
        .with_options(
            DiscoveryOptions::new().maximum_entries(Some(1)),
        );

        let report = discover(&request)
            .expect("discovery should execute");

        assert!(!report.is_complete());
        assert!(report.issues().iter().any(|issue| {
            issue.kind() == DiscoveryIssueKind::EntryBudgetExceeded
        }));
    }

    #[test]
    fn rejects_an_empty_root_request() {
        let request = DiscoveryRequest::new(
            std::iter::empty::<PathBuf>(),
        );

        assert_eq!(
            discover(&request).unwrap_err(),
            DiscoveryError::NoRoots,
        );
    }

    #[test]
    fn duplicate_roots_are_scanned_once() {
        let directory = TestDirectory::new();

        write_manifest(
            directory.path(),
            "org.example.once",
        );

        let request = DiscoveryRequest::new([
            directory.path().to_path_buf(),
            directory.path().to_path_buf(),
        ]);

        let report = discover(&request)
            .expect("discovery should execute");

        assert_eq!(report.roots().len(), 1);
        assert_eq!(report.candidates().len(), 1);
    }
}
