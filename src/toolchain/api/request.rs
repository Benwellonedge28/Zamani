
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Public Request Contracts
//!
//! Path: src/toolchain/api/request.rs
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//!
//! ## Purpose
//!
//! Defines the versioned, serializable request contracts accepted by the
//! Zamani toolchain API.
//!
//! Requests describe developer intent. They do not perform compilation,
//! dependency resolution, package installation, target selection,
//! hardware discovery, deployment, or execution.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - The public toolchain request envelope.
//! - Request context and client identification.
//! - Project and workspace location references.
//! - Target-selection intent.
//! - Operational budget declarations.
//! - Build, check, run, test, package, install, publish and deploy requests.
//! - Workspace-discovery requests.
//! - Request validation.
//! - Request classification and operation identification.
//!
//! It does not own:
//!
//! - Zamani source grammar or AST.
//! - Semantic requirements or capability analysis.
//! - Compiler configuration or compilation.
//! - Canonical target identities or target registration.
//! - Physical resource discovery or allocation.
//! - Dependency resolution.
//! - Package archive implementation.
//! - Registry protocols or authentication.
//! - External executable discovery or execution.
//! - Runtime scheduling or application execution.
//! - Deployment implementation.
//!
//! ## Existing integration authorities
//!
//! - crate::toolchain::ids
//! - crate::toolchain::version
//! - crate::toolchain::errors
//! - crate::zamani_project_config
//! - crate::compiler
//! - crate::backend
//! - crate::zhal
//! - crate::zur
//!
//! The toolchain coordinates these authorities without replacing them.
//!
//! ## POCO-REAF
//!
//! One logical request may produce multiple compatible target-specific
//! realizations.
//!
//! A target selection is an expression of intent, not proof of feasibility.
//! Feasibility must be established by the canonical compiler, capability,
//! backend, hardware and runtime authorities.
//!
//! No universal hardware or computational-domain limits are introduced.
//!
//! Literal infinite execution or storage is not promised.
//!
//! ## Validation boundary
//!
//! Serde deserialization is intentionally separate from semantic validation.
//! Every untrusted request MUST pass RequestEnvelope::validate before
//! dispatch. This prevents deserialized data from bypassing validation.
//!
//! Validation is structural only. Filesystem existence, permissions,
//! authentication, target compatibility, semantic correctness and actual
//! resource availability are checked by their authoritative services.
//!
//! ## Security
//!
//! Requests must never carry passwords, access tokens, private keys,
//! or other raw credentials.
//!
//! Registry authentication and secret resolution belong to the registered
//! authentication providers.
//!
//! Process arguments are data, not shell command strings.
//!
//! Paths are caller-supplied references. Their existence and security must
//! be checked by the service that consumes them.

use serde::{Deserialize, Serialize};

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::PathBuf;
use std::time::Duration;

use crate::toolchain::ids::{
    EnvironmentId,
    PackageId,
    ProfileId,
    ProjectId,
    RegistryId,
    WorkspaceId,
};

use crate::toolchain::version::{
    PackageFormatVersion,
    ToolchainApiVersion,
    INITIAL_TOOLCHAIN_API_VERSION,
};

// ============================================================================
// VALIDATION ERRORS
// ============================================================================

/// Structural request-validation failure.
///
/// The field identifies the invalid request field. The message describes
/// the violated invariant.
///
/// This is not a compiler, semantic-analysis or provider error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestValidationError {
    field: String,
    message: String,
}

impl RequestValidationError {
    /// Construct a validation error.
    fn new(
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Return the invalid field path.
    #[must_use]
    pub fn field(&self) -> &str {
        &self.field
    }

    /// Return the validation explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for RequestValidationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "invalid request field '{}': {}",
            self.field,
            self.message,
        )
    }
}

impl std::error::Error for RequestValidationError {}

/// Result of structural request validation.
pub type RequestValidationResult<T> =
    std::result::Result<T, RequestValidationError>;

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

/// Validate a non-empty external identifier.
///
/// Identifiers are intentionally open-ended so third-party providers and
/// future computational domains can introduce their own names.
///
/// Whitespace-only and control-character-containing identifiers are rejected.
/// No arbitrary identifier length or provider-count ceiling is introduced.
fn validate_identifier(
    field: &str,
    value: &str,
) -> RequestValidationResult<()> {
    if value.is_empty() || value.trim().is_empty() {
        return Err(RequestValidationError::new(
            field,
            "identifier must not be empty",
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(RequestValidationError::new(
            field,
            "identifier must not contain control characters",
        ));
    }

    if value != value.trim() {
        return Err(RequestValidationError::new(
            field,
            "identifier must not contain surrounding whitespace",
        ));
    }

    Ok(())
}

/// Validate a filesystem path without accessing the filesystem.
fn validate_path(
    field: &str,
    path: &std::path::Path,
) -> RequestValidationResult<()> {
    if path.as_os_str().is_empty() {
        return Err(RequestValidationError::new(
            field,
            "path must not be empty",
        ));
    }

    Ok(())
}

// ============================================================================
// CLIENT IDENTITY
// ============================================================================

/// Describes the API consumer issuing a request.
///
/// The kind is an open identifier rather than a closed enum. This allows
/// future IDEs, automation systems, agents, enterprise clients and other
/// integrations without changing the request contract.
///
/// This descriptor is informational. It does not authenticate the caller.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientIdentity {
    /// Open-ended client category.
    pub kind: String,

    /// Client name.
    pub name: String,

    /// Optional client implementation version.
    pub version: Option<String>,
}

impl ClientIdentity {
    /// Construct and validate a client identity.
    pub fn new(
        kind: impl Into<String>,
        name: impl Into<String>,
    ) -> RequestValidationResult<Self> {
        let identity = Self {
            kind: kind.into(),
            name: name.into(),
            version: None,
        };

        identity.validate()?;

        Ok(identity)
    }

    /// Set the client implementation version.
    pub fn with_version(
        mut self,
        version: impl Into<String>,
    ) -> RequestValidationResult<Self> {
        self.version = Some(version.into());
        self.validate()?;
        Ok(self)
    }

    /// Validate the client descriptor.
    pub fn validate(&self) -> RequestValidationResult<()> {
        validate_identifier("client.kind", &self.kind)?;
        validate_identifier("client.name", &self.name)?;

        if let Some(version) = &self.version {
            validate_identifier("client.version", version)?;
        }

        Ok(())
    }
}

// ============================================================================
// REQUEST CONTEXT
// ============================================================================

/// Metadata shared by every toolchain request.
///
/// Request identity is separate from project, package, task and build
/// identities. It identifies one API invocation, not a persistent resource.
///
/// `idempotency_key` is an optional caller-provided key. Services must
/// implement operation-specific idempotency before treating repeated
/// requests as the same operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestContext {
    /// Toolchain API contract requested by the client.
    pub api_version: ToolchainApiVersion,

    /// Unique request identifier within the caller's chosen namespace.
    pub request_id: String,

    /// Optional correlation identifier for tracing related operations.
    pub correlation_id: Option<String>,

    /// Optional idempotency key.
    pub idempotency_key: Option<String>,

    /// Description of the calling client.
    pub client: ClientIdentity,
}

impl RequestContext {
    /// Construct a request context using the initial toolchain API version.
    ///
    /// This constructor does not claim that the API implementation has
    /// completed every operation represented by the API.
    pub fn new(
        request_id: impl Into<String>,
        client: ClientIdentity,
    ) -> RequestValidationResult<Self> {
        let context = Self {
            api_version: INITIAL_TOOLCHAIN_API_VERSION,
            request_id: request_id.into(),
            correlation_id: None,
            idempotency_key: None,
            client,
        };

        context.validate()?;

        Ok(context)
    }

    /// Set a correlation identifier.
    pub fn with_correlation_id(
        mut self,
        value: impl Into<String>,
    ) -> RequestValidationResult<Self> {
        self.correlation_id = Some(value.into());
        self.validate()?;
        Ok(self)
    }

    /// Set an idempotency key.
    pub fn with_idempotency_key(
        mut self,
        value: impl Into<String>,
    ) -> RequestValidationResult<Self> {
        self.idempotency_key = Some(value.into());
        self.validate()?;
        Ok(self)
    }

    /// Validate request metadata.
    pub fn validate(&self) -> RequestValidationResult<()> {
        validate_identifier("request_id", &self.request_id)?;
        self.client.validate()?;

        if let Some(value) = &self.correlation_id {
            validate_identifier("correlation_id", value)?;
        }

        if let Some(value) = &self.idempotency_key {
            validate_identifier("idempotency_key", value)?;
        }

        Ok(())
    }
}

// ============================================================================
// PROJECT REFERENCES
// ============================================================================

/// Identifies a project without requiring filesystem access.
///
/// A project can be identified by its existing toolchain identity or by
/// an explicitly supplied project-root path.
///
/// The path variant does not automatically discover or create a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ProjectLocator {
    /// Locate an already identified project.
    Id(ProjectId),

    /// Locate a project using its root directory.
    Path(PathBuf),
}

impl ProjectLocator {
    /// Construct a path-based project locator.
    pub fn from_path(
        path: impl Into<PathBuf>,
    ) -> RequestValidationResult<Self> {
        let locator = Self::Path(path.into());
        locator.validate()?;
        Ok(locator)
    }

    /// Validate the locator's structural representation.
    pub fn validate(&self) -> RequestValidationResult<()> {
        match self {
            Self::Id(_) => Ok(()),
            Self::Path(path) => validate_path("project.path", path),
        }
    }
}

/// Identifies the project and optional workspace associated with an
/// operation.
///
/// The manifest remains owned by `zamani_project_config`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectScope {
    /// Project being operated on.
    pub project: ProjectLocator,

    /// Optional containing workspace.
    pub workspace: Option<WorkspaceId>,
}

impl ProjectScope {
    /// Construct a project scope.
    pub fn new(project: ProjectLocator) -> RequestValidationResult<Self> {
        let scope = Self {
            project,
            workspace: None,
        };

        scope.validate()?;
        Ok(scope)
    }

    /// Associate the project with a workspace.
    pub fn with_workspace(mut self, workspace: WorkspaceId) -> Self {
        self.workspace = Some(workspace);
        self
    }

    /// Validate the project scope.
    pub fn validate(&self) -> RequestValidationResult<()> {
        self.project.validate()
    }
}

// ============================================================================
// TARGET-SELECTION INTENT
// ============================================================================

/// An opaque target selector supplied by the caller.
///
/// The selector is not a physical hardware identity or a canonical target
/// descriptor. It is resolved by the authoritative target registry.
///
/// Namespaces permit extensible identifiers for future architectures,
/// domains, vendors and third-party target providers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TargetSelector(String);

impl TargetSelector {
    /// Construct a target selector.
    pub fn new(
        value: impl Into<String>,
    ) -> RequestValidationResult<Self> {
        let selector = Self(value.into());
        selector.validate()?;
        Ok(selector)
    }

    /// Return the opaque selector.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Validate the selector.
    pub fn validate(&self) -> RequestValidationResult<()> {
        validate_identifier("target_selector", &self.0)
    }
}

impl fmt::Display for TargetSelector {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Describes how target intent should be interpreted.
///
/// Explicit targets are requirements of the request.
///
/// Capability-based resolution permits the downstream planner to discover
/// compatible realizations. Preferences must never silently become
/// mandatory requirements.
///
/// Neither variant claims that a requested target is feasible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TargetSelection {
    /// Require the specified targets to be considered.
    Explicit {
        /// Target selectors.
        targets: Vec<TargetSelector>,

        /// Whether every requested target must be realized.
        require_all: bool,
    },

    /// Resolve compatible targets using canonical capability information.
    ResolveFromCapabilities {
        /// Optional ordered preferences.
        ///
        /// An empty list permits the downstream policy to determine
        /// eligible realizations. It does not mean native execution.
        preferences: Vec<TargetSelector>,
    },
}

impl TargetSelection {
    /// Construct an explicit target selection.
    pub fn explicit(
        targets: Vec<TargetSelector>,
        require_all: bool,
    ) -> RequestValidationResult<Self> {
        let selection = Self::Explicit {
            targets,
            require_all,
        };

        selection.validate()?;
        Ok(selection)
    }

    /// Request capability-based target resolution.
    pub fn resolve_from_capabilities(
        preferences: Vec<TargetSelector>,
    ) -> RequestValidationResult<Self> {
        let selection = Self::ResolveFromCapabilities {
            preferences,
        };

        selection.validate()?;
        Ok(selection)
    }

    /// Validate target-selection intent.
    pub fn validate(&self) -> RequestValidationResult<()> {
        let (targets, allow_empty) = match self {
            Self::Explicit { targets, .. } => (targets, false),
            Self::ResolveFromCapabilities { preferences } => {
                (preferences, true)
            }
        };

        if targets.is_empty() && !allow_empty {
            return Err(RequestValidationError::new(
                "target_selection.targets",
                "explicit target selection must contain at least one target",
            ));
        }

        let mut seen = BTreeSet::new();

        for target in targets {
            target.validate()?;

            if !seen.insert(target) {
                return Err(RequestValidationError::new(
                    "target_selection.targets",
                    "duplicate target selector",
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// EXTENSIBLE OPERATIONAL BUDGETS
// ============================================================================

/// Validated, open-ended operational budget key.
///
/// Examples:
///
/// - build.wall_time_ms
/// - build.memory_bytes
/// - compiler.work_units
/// - deployment.network_bytes
/// - quantum.logical_qubits
/// - distributed.participants
///
/// These are operational request budgets, not language-level capabilities
/// or universal hardware limits.
///
/// The authoritative budget subsystem defines the meaning and unit of each
/// registered key.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BudgetKey(String);

impl BudgetKey {
    /// Construct a budget key.
    pub fn new(
        value: impl Into<String>,
    ) -> RequestValidationResult<Self> {
        let key = Self(value.into());
        key.validate()?;
        Ok(key)
    }

    /// Borrow the canonical key.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Validate the key.
    pub fn validate(&self) -> RequestValidationResult<()> {
        validate_identifier("budgets.key", &self.0)?;

        if !self.0.contains('.') {
            return Err(RequestValidationError::new(
                "budgets.key",
                "budget keys must use a namespace-qualified name",
            ));
        }

        Ok(())
    }
}

/// Configurable operational budgets requested for an operation.
///
/// An absent key means that the request supplies no explicit limit for
/// that dimension.
///
/// A value of zero is a genuine zero budget, not unlimited capacity.
///
/// The map is open-ended. New computational domains can add budget
/// dimensions without changing this request schema.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OperationalBudgets(BTreeMap<BudgetKey, u128>);

impl OperationalBudgets {
    /// Construct an empty budget declaration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace a budget.
    ///
    /// Returns the previous value when one existed.
    pub fn set(
        &mut self,
        key: BudgetKey,
        value: u128,
    ) -> Option<u128> {
        self.0.insert(key, value)
    }

    /// Retrieve a budget.
    #[must_use]
    pub fn get(&self, key: &BudgetKey) -> Option<u128> {
        self.0.get(key).copied()
    }

    /// Remove a budget.
    pub fn remove(&mut self, key: &BudgetKey) -> Option<u128> {
        self.0.remove(key)
    }

    /// Iterate over budgets in deterministic key order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&BudgetKey, &u128)> {
        self.0.iter()
    }

    /// Return whether no explicit budgets were supplied.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Validate all budget keys.
    pub fn validate(&self) -> RequestValidationResult<()> {
        for key in self.0.keys() {
            key.validate()?;
        }

        Ok(())
    }
}

// ============================================================================
// COMMON BUILD REQUEST
// ============================================================================

/// A complete logical build request.
///
/// This describes one logical build operation. It may result in multiple
/// target-specific artifacts when the requested realizations are compatible.
///
/// Compiler configuration, semantic requirements, compiler passes and
/// backend settings remain owned by their authoritative subsystems.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildRequest {
    /// Project to build.
    pub scope: ProjectScope,

    /// Requested target realizations.
    pub targets: TargetSelection,

    /// Optional existing toolchain profile identity.
    pub profile: Option<ProfileId>,

    /// Open-ended feature identifiers.
    pub features: BTreeSet<String>,

    /// Additional operational budgets.
    pub budgets: OperationalBudgets,
}

impl BuildRequest {
    /// Construct a build request.
    pub fn new(
        scope: ProjectScope,
        targets: TargetSelection,
    ) -> RequestValidationResult<Self> {
        let request = Self {
            scope,
            targets,
            profile: None,
            features: BTreeSet::new(),
            budgets: OperationalBudgets::new(),
        };

        request.validate()?;
        Ok(request)
    }

    /// Set a build profile.
    pub fn with_profile(mut self, profile: ProfileId) -> Self {
        self.profile = Some(profile);
        self
    }

    /// Replace the requested feature set.
    pub fn with_features(
        mut self,
        features: BTreeSet<String>,
    ) -> RequestValidationResult<Self> {
        self.features = features;
        self.validate()?;
        Ok(self)
    }

    /// Replace the operational budget declaration.
    pub fn with_budgets(
        mut self,
        budgets: OperationalBudgets,
    ) -> RequestValidationResult<Self> {
        self.budgets = budgets;
        self.validate()?;
        Ok(self)
    }

    /// Validate the complete build request.
    pub fn validate(&self) -> RequestValidationResult<()> {
        self.scope.validate()?;
        self.targets.validate()?;
        self.budgets.validate()?;

        for feature in &self.features {
            validate_identifier("features", feature)?;
        }

        Ok(())
    }
}

// ============================================================================
// WORKSPACE DISCOVERY
// ============================================================================

/// Request to discover a Zamani workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceDiscoveryRequest {
    /// Starting directory for discovery.
    pub root: PathBuf,

    /// Whether parent directories may be examined.
    ///
    /// This is an explicit traversal policy, not permission to access
    /// arbitrary filesystem locations.
    pub search_parents: bool,
}

impl WorkspaceDiscoveryRequest {
    /// Construct a workspace-discovery request.
    pub fn new(
        root: impl Into<PathBuf>,
        search_parents: bool,
    ) -> RequestValidationResult<Self> {
        let request = Self {
            root: root.into(),
            search_parents,
        };

        request.validate()?;
        Ok(request)
    }

    /// Validate the discovery request.
    pub fn validate(&self) -> RequestValidationResult<()> {
        validate_path("workspace_discovery.root", &self.root)
    }
}

// ============================================================================
// CHECK REQUEST
// ============================================================================

/// Request to validate a project and its compilation requirements.
///
/// A check must report actual validation results. It must not claim that
/// compilation or execution succeeded unless those operations were performed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckRequest {
    /// Project scope.
    pub scope: ProjectScope,

    /// Optional target compatibility checks.
    pub targets: Option<TargetSelection>,

    /// Optional compilation profile.
    pub profile: Option<ProfileId>,

    /// Operational budgets.
    pub budgets: OperationalBudgets,
}

impl CheckRequest {
    /// Construct a check request.
    pub fn new(scope: ProjectScope) -> RequestValidationResult<Self> {
        let request = Self {
            scope,
            targets: None,
            profile: None,
            budgets: OperationalBudgets::new(),
        };

        request.validate()?;
        Ok(request)
    }

    /// Validate the request.
    pub fn validate(&self) -> RequestValidationResult<()> {
        self.scope.validate()?;

        if let Some(targets) = &self.targets {
            targets.validate()?;
        }

        self.budgets.validate()
    }
}

// ============================================================================
// RUN REQUEST
// ============================================================================

/// Request to build and run a project.
///
/// Running is a distinct operation from building. The service must validate
/// the produced artifact and execution environment before handing execution
/// to the canonical runtime.
///
/// Arguments are individual strings, never shell command text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunRequest {
    /// Build specification used to produce the executable artifact.
    pub build: BuildRequest,

    /// Program arguments, in their original order.
    pub arguments: Vec<String>,

    /// Optional working directory for the application.
    pub working_directory: Option<PathBuf>,
}

impl RunRequest {
    /// Construct a run request.
    pub fn new(build: BuildRequest) -> RequestValidationResult<Self> {
        let request = Self {
            build,
            arguments: Vec::new(),
            working_directory: None,
        };

        request.validate()?;
        Ok(request)
    }

    /// Replace program arguments.
    pub fn with_arguments(
        mut self,
        arguments: Vec<String>,
    ) -> RequestValidationResult<Self> {
        self.arguments = arguments;
        self.validate()?;
        Ok(self)
    }

    /// Set the application's working directory.
    pub fn with_working_directory(
        mut self,
        path: impl Into<PathBuf>,
    ) -> RequestValidationResult<Self> {
        self.working_directory = Some(path.into());
        self.validate()?;
        Ok(self)
    }

    /// Validate the request.
    pub fn validate(&self) -> RequestValidationResult<()> {
        self.build.validate()?;

        if let Some(path) = &self.working_directory {
            validate_path("run.working_directory", path)?;
        }

        if self.arguments.iter().any(|argument| {
            argument.contains('\0')
        }) {
            return Err(RequestValidationError::new(
                "run.arguments",
                "arguments must not contain NUL characters",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// TEST REQUEST
// ============================================================================

/// Request to execute project tests.
///
/// An empty selector set means that the test service should apply its
/// registered test-discovery policy. It does not mean zero tests exist.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestRequest {
    /// Project scope.
    pub scope: ProjectScope,

    /// Optional target realization.
    pub targets: Option<TargetSelection>,

    /// Optional profile.
    pub profile: Option<ProfileId>,

    /// Explicit test selectors.
    pub selectors: BTreeSet<String>,

    /// Whether test discovery should include dependent workspace members.
    pub include_workspace_members: bool,

    /// Operational budgets.
    pub budgets: OperationalBudgets,
}

impl TestRequest {
    /// Construct a test request.
    pub fn new(scope: ProjectScope) -> RequestValidationResult<Self> {
        let request = Self {
            scope,
            targets: None,
            profile: None,
            selectors: BTreeSet::new(),
            include_workspace_members: false,
            budgets: OperationalBudgets::new(),
        };

        request.validate()?;
        Ok(request)
    }

    /// Validate the test request.
    pub fn validate(&self) -> RequestValidationResult<()> {
        self.scope.validate()?;

        if let Some(targets) = &self.targets {
            targets.validate()?;
        }

        self.budgets.validate()?;

        for selector in &self.selectors {
            validate_identifier("test.selectors", selector)?;
        }

        Ok(())
    }
}

// ============================================================================
// PACKAGE CREATION REQUEST
// ============================================================================

/// Request to create a package from a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageRequest {
    /// Source project.
    pub scope: ProjectScope,

    /// Requested package archive format contract.
    pub format_version: PackageFormatVersion,

    /// Whether source files should be included.
    pub include_source: bool,

    /// Optional destination for the generated archive.
    pub output_path: Option<PathBuf>,
}

impl PackageRequest {
    /// Construct a package-creation request.
    pub fn new(
        scope: ProjectScope,
        format_version: PackageFormatVersion,
    ) -> RequestValidationResult<Self> {
        let request = Self {
            scope,
            format_version,
            include_source: true,
            output_path: None,
        };

        request.validate()?;
        Ok(request)
    }

    /// Validate the package request.
    pub fn validate(&self) -> RequestValidationResult<()> {
        self.scope.validate()?;

        if let Some(path) = &self.output_path {
            validate_path("package.output_path", path)?;
        }

        Ok(())
    }
}

// ============================================================================
// PACKAGE SOURCE
// ============================================================================

/// Describes where a package should be obtained.
///
/// Registry access is performed by registered registry providers.
/// This request type does not perform network access.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PackageSource {
    /// An existing local package archive.
    LocalArchive(PathBuf),

    /// A package identified through a configured registry.
    Registry {
        /// Registry identity.
        registry: RegistryId,

        /// Package identity.
        package: PackageId,

        /// Version constraint interpreted by dependency resolution.
        version_requirement: String,
    },
}

impl PackageSource {
    /// Validate the package source.
    pub fn validate(&self) -> RequestValidationResult<()> {
        match self {
            Self::LocalArchive(path) => {
                validate_path("package_source.local_archive", path)
            }

            Self::Registry {
                version_requirement,
                ..
            } => validate_identifier(
                "package_source.version_requirement",
                version_requirement,
            ),
        }
    }
}

// ============================================================================
// PACKAGE INSTALLATION REQUEST
// ============================================================================

/// Request to install a package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstallRequest {
    /// Package source.
    pub source: PackageSource,

    /// Optional explicitly configured installation destination.
    ///
    /// The package installation service owns filesystem operations and
    /// must protect against path traversal and conflicting installations.
    pub destination: Option<PathBuf>,
}

impl InstallRequest {
    /// Construct an installation request.
    pub fn new(source: PackageSource) -> RequestValidationResult<Self> {
        let request = Self {
            source,
            destination: None,
        };

        request.validate()?;
        Ok(request)
    }

    /// Validate the installation request.
    pub fn validate(&self) -> RequestValidationResult<()> {
        self.source.validate()?;

        if let Some(destination) = &self.destination {
            validate_path("install.destination", destination)?;
        }

        Ok(())
    }
}

// ============================================================================
// PACKAGE PUBLISHING REQUEST
// ============================================================================

/// Request to publish a package archive.
///
/// Publishing is an externally visible operation. The service must
/// authenticate and authorize the caller, verify package integrity,
/// and implement safe retry/idempotency behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublishRequest {
    /// Package archive to publish.
    pub archive_path: PathBuf,

    /// Destination registry.
    pub registry: RegistryId,

    /// Whether the registry operation should perform validation only.
    pub dry_run: bool,
}

impl PublishRequest {
    /// Construct a publishing request.
    pub fn new(
        archive_path: impl Into<PathBuf>,
        registry: RegistryId,
    ) -> RequestValidationResult<Self> {
        let request = Self {
            archive_path: archive_path.into(),
            registry,
            dry_run: false,
        };

        request.validate()?;
        Ok(request)
    }

    /// Validate the publishing request.
    pub fn validate(&self) -> RequestValidationResult<()> {
        validate_path("publish.archive_path", &self.archive_path)
    }
}

// ============================================================================
// DEPLOYMENT REQUEST
// ============================================================================

/// Request to prepare or perform a deployment.
///
/// Physical placement, infrastructure allocation and execution remain
/// with deployment providers, ZHAL and ZUR.
///
/// A successful request validation does not mean deployment is feasible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeployRequest {
    /// Project to deploy.
    pub scope: ProjectScope,

    /// Requested target realizations.
    pub targets: TargetSelection,

    /// Optional explicitly selected environment.
    pub environment: Option<EnvironmentId>,

    /// Optional deployment profile.
    pub profile: Option<ProfileId>,

    /// Whether the caller requests planning without deployment side effects.
    pub plan_only: bool,

    /// Operational budgets.
    pub budgets: OperationalBudgets,
}

impl DeployRequest {
    /// Construct a deployment request.
    pub fn new(
        scope: ProjectScope,
        targets: TargetSelection,
    ) -> RequestValidationResult<Self> {
        let request = Self {
            scope,
            targets,
            environment: None,
            profile: None,
            plan_only: true,
            budgets: OperationalBudgets::new(),
        };

        request.validate()?;
        Ok(request)
    }

    /// Validate the deployment request.
    pub fn validate(&self) -> RequestValidationResult<()> {
        self.scope.validate()?;
        self.targets.validate()?;
        self.budgets.validate()
    }
}

// ============================================================================
// OPERATION DISPATCH CONTRACT
// ============================================================================

/// One supported request category.
///
/// This enum identifies API operation contracts. It does not guarantee
/// that the corresponding service implementation or provider is installed.
///
/// New operation categories can be added without changing the identities
/// or semantics of existing operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "operation", content = "request", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ToolchainOperation {
    /// Discover a workspace.
    DiscoverWorkspace(WorkspaceDiscoveryRequest),

    /// Build a project.
    Build(BuildRequest),

    /// Check a project.
    Check(CheckRequest),

    /// Build and run a project.
    Run(RunRequest),

    /// Execute project tests.
    Test(TestRequest),

    /// Create a package.
    Package(PackageRequest),

    /// Install a package.
    Install(InstallRequest),

    /// Publish a package.
    Publish(PublishRequest),

    /// Deploy a project.
    Deploy(DeployRequest),
}

impl ToolchainOperation {
    /// Return the stable operation identifier.
    #[must_use]
    pub const fn operation_name(&self) -> &'static str {
        match self {
            Self::DiscoverWorkspace(_) => "workspace.discover",
            Self::Build(_) => "build",
            Self::Check(_) => "check",
            Self::Run(_) => "run",
            Self::Test(_) => "test",
            Self::Package(_) => "package",
            Self::Install(_) => "package.install",
            Self::Publish(_) => "package.publish",
            Self::Deploy(_) => "deploy",
        }
    }

    /// Whether the operation can produce externally visible side effects.
    ///
    /// This is descriptive metadata, not an authorization decision.
    #[must_use]
    pub const fn may_mutate_external_state(&self) -> bool {
        matches!(
            self,
            Self::Install(_)
                | Self::Publish(_)
                | Self::Deploy(_)
        )
    }

    /// Validate the selected operation and its complete payload.
    pub fn validate(&self) -> RequestValidationResult<()> {
        match self {
            Self::DiscoverWorkspace(request) => request.validate(),
            Self::Build(request) => request.validate(),
            Self::Check(request) => request.validate(),
            Self::Run(request) => request.validate(),
            Self::Test(request) => request.validate(),
            Self::Package(request) => request.validate(),
            Self::Install(request) => request.validate(),
            Self::Publish(request) => request.validate(),
            Self::Deploy(request) => request.validate(),
        }
    }
}

// ============================================================================
// PUBLIC REQUEST ENVELOPE
// ============================================================================

/// Complete request submitted to the Zamani toolchain API.
///
/// All API clients use this envelope:
///
/// - CLI.
/// - IDE.
/// - Editor.
/// - Human developer.
/// - Automation.
/// - AI agent.
/// - Enterprise integration.
/// - Third-party developer integration.
///
/// Every client receives the same request validation and operation
/// authorization rules.
///
/// Request envelopes are serializable and can be transported through
/// future local or remote API adapters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestEnvelope {
    /// Common request metadata.
    pub context: RequestContext,

    /// Requested operation and its payload.
    pub operation: ToolchainOperation,
}

impl RequestEnvelope {
    /// Construct a complete request envelope.
    pub fn new(
        context: RequestContext,
        operation: ToolchainOperation,
    ) -> RequestValidationResult<Self> {
        let envelope = Self {
            context,
            operation,
        };

        envelope.validate()?;
        Ok(envelope)
    }

    /// Return the operation identifier.
    #[must_use]
    pub fn operation_name(&self) -> &'static str {
        self.operation.operation_name()
    }

    /// Determine whether the operation may mutate external state.
    #[must_use]
    pub fn may_mutate_external_state(&self) -> bool {
        self.operation.may_mutate_external_state()
    }

    /// Validate the complete request before service dispatch.
    ///
    /// This method MUST be called for both locally constructed and
    /// deserialized requests.
    ///
    /// API version compatibility, authentication, authorization,
    /// filesystem validation, semantic validation, target feasibility,
    /// and provider availability are separate service-level checks.
    pub fn validate(&self) -> RequestValidationResult<()> {
        self.context.validate()?;
        self.operation.validate()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::ids::IdNamespace;

    fn project_id() -> ProjectId {
        ProjectId::new(
            IdNamespace::new("test").expect("valid namespace"),
            1,
        )
        .expect("valid project identity")
    }

    fn workspace_id() -> WorkspaceId {
        WorkspaceId::new(
            IdNamespace::new("test").expect("valid namespace"),
            2,
        )
        .expect("valid workspace identity")
    }

    fn profile_id() -> ProfileId {
        ProfileId::new(
            IdNamespace::new("test").expect("valid namespace"),
            3,
        )
        .expect("valid profile identity")
    }

    fn client() -> ClientIdentity {
        ClientIdentity::new("cli", "zamani-pkg")
            .expect("valid client")
    }

    fn context() -> RequestContext {
        RequestContext::new("request-1", client())
            .expect("valid context")
    }

    fn target() -> TargetSelector {
        TargetSelector::new("provider:target")
            .expect("valid target")
    }

    fn scope() -> ProjectScope {
        ProjectScope::new(ProjectLocator::Id(project_id()))
            .expect("valid project scope")
            .with_workspace(workspace_id())
    }

    fn build_request() -> BuildRequest {
        BuildRequest::new(
            scope(),
            TargetSelection::explicit(
                vec![target()],
                true,
            )
            .expect("valid target selection"),
        )
        .expect("valid build request")
        .with_profile(profile_id())
    }

    #[test]
    fn request_envelope_accepts_valid_build_request() {
        let envelope = RequestEnvelope::new(
            context(),
            ToolchainOperation::Build(build_request()),
        )
        .expect("valid request");

        assert_eq!(envelope.operation_name(), "build");
        assert!(!envelope.may_mutate_external_state());
        assert!(envelope.validate().is_ok());
    }

    #[test]
    fn request_context_requires_request_identity() {
        let result = RequestContext::new("", client());

        assert!(result.is_err());
    }

    #[test]
    fn target_selection_requires_explicit_targets() {
        let result = TargetSelection::explicit(
            Vec::new(),
            true,
        );

        assert!(result.is_err());
    }

    #[test]
    fn capability_resolution_allows_empty_preferences() {
        let selection =
            TargetSelection::resolve_from_capabilities(
                Vec::new(),
            )
            .expect("empty preferences are valid");

        assert!(selection.validate().is_ok());
    }

    #[test]
    fn duplicate_explicit_targets_are_rejected() {
        let selector = target();

        let result = TargetSelection::explicit(
            vec![selector.clone(), selector],
            true,
        );

        assert!(result.is_err());
    }

    #[test]
    fn target_selectors_remain_extensible() {
        for value in [
            "classical:native",
            "quantum:simulator",
            "hdl:synthesis",
            "ai:tensor",
            "hybrid:execution",
            "distributed:cluster",
            "third-party:future-target",
        ] {
            assert!(TargetSelector::new(value).is_ok());
        }
    }

    #[test]
    fn operational_budgets_are_open_ended() {
        let mut budgets = OperationalBudgets::new();

        let key = BudgetKey::new("quantum.logical_qubits")
            .expect("valid budget key");

        assert_eq!(budgets.set(key.clone(), 1024), None);
        assert_eq!(budgets.get(&key), Some(1024));
        assert_eq!(budgets.remove(&key), Some(1024));
        assert!(budgets.is_empty());
    }

    #[test]
    fn zero_budget_is_not_unlimited() {
        let mut budgets = OperationalBudgets::new();

        let key = BudgetKey::new("build.work_units")
            .expect("valid budget key");

        budgets.set(key.clone(), 0);

        assert_eq!(budgets.get(&key), Some(0));
    }

    #[test]
    fn namespaced_budget_keys_are_required() {
        assert!(BudgetKey::new("memory").is_err());
        assert!(BudgetKey::new("build.memory_bytes").is_ok());
    }

    #[test]
    fn package_installation_is_marked_as_mutating() {
        let source = PackageSource::LocalArchive(
            PathBuf::from("package.zpkg"),
        );

        let request = InstallRequest::new(source)
            .expect("valid install request");

        let envelope = RequestEnvelope::new(
            context(),
            ToolchainOperation::Install(request),
        )
        .expect("valid envelope");

        assert!(envelope.may_mutate_external_state());
    }

    #[test]
    fn package_publishing_is_marked_as_mutating() {
        let registry = RegistryId::new(
            IdNamespace::new("test").expect("namespace"),
            4,
        )
        .expect("registry identity");

        let request = PublishRequest::new(
            "package.zpkg",
            registry,
        )
        .expect("publish request");

        let envelope = RequestEnvelope::new(
            context(),
            ToolchainOperation::Publish(request),
        )
        .expect("valid envelope");

        assert!(envelope.may_mutate_external_state());
    }

    #[test]
    fn deployment_can_be_requested_as_plan_only() {
        let request = DeployRequest::new(
            scope(),
            TargetSelection::resolve_from_capabilities(
                Vec::new(),
            )
            .expect("target selection"),
        )
        .expect("deployment request");

        assert!(request.plan_only);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn request_envelope_serializes_and_deserializes() {
        let envelope = RequestEnvelope::new(
            context(),
            ToolchainOperation::Build(build_request()),
        )
        .expect("valid request");

        let serialized = serde_json::to_string(&envelope)
            .expect("serialization");

        let decoded: RequestEnvelope =
            serde_json::from_str(&serialized)
                .expect("deserialization");

        assert_eq!(decoded, envelope);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn deserialized_invalid_requests_are_rejected_at_boundary() {
        let json = r#"
        {
            "context": {
                "api_version": "1.0.0",
                "request_id": "",
                "correlation_id": null,
                "idempotency_key": null,
                "client": {
                    "kind": "cli",
                    "name": "zamani",
                    "version": null
                }
            },
            "operation": {
                "operation": "build",
                "request": {
                    "scope": {
                        "project": {
                            "id": "test:1"
                        },
                        "workspace": null
                    },
                    "targets": {
                        "explicit": {
                            "targets": [],
                            "require_all": true
                        }
                    },
                    "profile": null,
                    "features": [],
                    "budgets": {}
                }
            }
        }
        "#;

        // Deserialization is not a substitute for validation.
        let decoded: RequestEnvelope =
            serde_json::from_str(json)
                .expect("wire structure is deserializable");

        assert!(decoded.validate().is_err());
    }

    #[test]
    fn malformed_paths_are_rejected() {
        let result = ProjectLocator::from_path(PathBuf::new());

        assert!(result.is_err());
    }

    #[test]
    fn shell_interpretation_is_not_part_of_arguments() {
        let request = RunRequest::new(build_request())
            .expect("run request")
            .with_arguments(vec![
                "--input".to_owned(),
                "file with spaces.zm".to_owned(),
                "; not a shell command".to_owned(),
            ])
            .expect("arguments");

        assert!(request.validate().is_ok());
    }

    #[test]
    fn run_arguments_reject_nul_characters() {
        let result = RunRequest::new(build_request())
            .expect("run request")
            .with_arguments(vec!["invalid\0argument".to_owned()]);

        assert!(result.is_err());
    }

    #[test]
    fn workspace_discovery_requires_a_root() {
        assert!(WorkspaceDiscoveryRequest::new(
            PathBuf::new(),
            true,
        )
        .is_err());
    }

    #[test]
    fn operation_names_are_stable() {
        let request = ToolchainOperation::Build(build_request());

        assert_eq!(request.operation_name(), "build");
    }

    #[test]
    fn client_kinds_are_not_closed() {
        for kind in [
            "cli",
            "ide",
            "automation",
            "ai-agent",
            "enterprise",
            "third-party",
        ] {
            assert!(ClientIdentity::new(kind, "client").is_ok());
        }
    }
}
