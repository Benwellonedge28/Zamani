#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Extension Validation
//!
//! Path: `src/frontend/semantic/domains/extensions.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Validate semantic-extension manifests before their definitions are made
//! available to semantic analysis. This includes extension identity, version
//! compatibility, declared dependencies, conflicts, stability policy,
//! contribution identity, and dependency ordering.
//!
//! ## AUTHORITY
//!
//! - `grammar/Zamani.g4`
//! - `grammar/specification/extensibility.md`
//! - `grammar/specification/domains.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/config.rs`
//! - `src/frontend/semantic/context.rs`
//! - `src/frontend/semantic/domains/registry.rs`
//!
//! The grammar defines accepted syntax. Normative specifications define
//! extension meaning and compatibility. The canonical AST owns source
//! structure. The domain registry owns computational-domain definitions.
//! This module validates extension metadata and dependency relationships.
//!
//! ## OWNS
//!
//! - Namespace-qualified extension and contribution identities.
//! - Extension version and language-version compatibility declarations.
//! - Extension manifest metadata and validation.
//! - Dependency, conflict, and duplicate-contribution checks.
//! - Stability and extension-policy validation.
//! - Deterministic dependency ordering and cycle reporting.
//! - Structured extension-validation issues.
//!
//! ## DOES_NOT_OWN
//!
//! - Lexing, parsing, grammar interpretation, or AST construction.
//! - A replacement AST or source-level declaration model.
//! - Ordinary name resolution, type inference, or type compatibility.
//! - Effect inference, effect authorization, or capability evidence.
//! - The canonical computational-domain registry.
//! - Hardware discovery, resource allocation, or target feasibility.
//! - Extension code loading, execution, sandboxing, or trust verification.
//! - Classical IR, `quantum::ir`, HDL IR, lowering, scheduling, or runtime.
//!
//! ## INTEGRATION CONTRACT
//!
//! 1. The canonical AST adapter translates source declarations into manifests.
//! 2. `config.rs` supplies the active extension policy and language version.
//! 3. The semantic pipeline validates manifests before activating their
//!    semantic contributions.
//! 4. `domains/registry.rs` remains authoritative for domain identities.
//! 5. Shared symbol, type, effect, capability, resource, contract, and
//!    diagnostic services remain authoritative for their respective rules.
//! 6. The caller maps `ExtensionIssue` values to the shared diagnostic format,
//!    preserving the optional source `NodeId`.
//! 7. Extension implementations are dispatched only after their metadata,
//!    dependencies, compatibility, and activation policy have been checked.
//! 8. Downstream IR boundaries remain unchanged.
//!
//! This module intentionally does not assume unverified constructors for AST
//! extension declarations or shared diagnostics.
//!
//! ## SCALABILITY
//!
//! There is no language-level maximum for extensions, dependencies,
//! contributions, namespace depth, or declared resource requirements.
//! Dynamically sized standard-library collections are used.
//!
//! A finite process cannot analyze literally infinite input. Actual work is
//! bounded by available memory, time, representable values, and operational
//! budgets enforced by the semantic session. This module does not impose
//! machine-capacity ceilings or silently truncate input.
//!
//! ## DETERMINISM
//!
//! Ordered maps and sets define stable identity and traversal order.
//! The same manifest set, language version, and policy produce the same
//! ordered issues and dependency plan, independent of input ordering where
//! ordering has no declared semantic meaning.
//!
//! ## SECURITY
//!
//! Manifests are untrusted metadata. Validation does not load or execute
//! extension code, access the filesystem or network, inspect hardware,
//! or use global mutable state. Metadata validation alone is not proof of
//! publisher identity, code integrity, sandbox safety, or implementation
//! correctness; those are separate security and release responsibilities.
//!
//! ## FREEZE CRITERIA
//!
//! - The AST adapter preserves every supported extension declaration.
//! - The pipeline supplies the actual language version and extension policy.
//! - The manifest validator runs before extension activation.
//! - Dependency and contribution identities are unambiguous.
//! - Missing dependencies, incompatible versions, conflicts, and cycles
//!   cannot be treated as successful validation.
//! - Issues are mapped into shared diagnostics without losing source identity.
//! - Unit, integration, determinism, fuzz, and stress tests pass.
//! - The full workspace builds, formats, and passes required lint and test
//!   checks on the supported Rust toolchain.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::semantic::config::ExtensionPolicy;

/// Schema version of this module's manifest and validation contract.
///
/// This is independent of the language, AST, compiler, registry, and IR
/// versions.
pub const EXTENSION_VALIDATION_SCHEMA_VERSION: u16 = 1;

/// Result of validating an extension manifest set.
pub type ExtensionValidationResult<T> = Result<T, ExtensionInputError>;

/// Validate one identity component without imposing a language-level length
/// or namespace-depth ceiling.
fn validate_component(value: &str, label: &'static str) -> ExtensionValidationResult<()> {
    if value.trim().is_empty() {
        return Err(ExtensionInputError::EmptyIdentityComponent { label });
    }

    if value.chars().any(char::is_control) {
        return Err(ExtensionInputError::ControlCharacterInIdentity { label });
    }

    if value.chars().any(char::is_whitespace) {
        return Err(ExtensionInputError::WhitespaceInIdentity { label });
    }

    Ok(())
}

/// Namespace-qualified identity shared by extensions, contributions, and
/// capability references.
///
/// The two components are retained separately. This avoids parsing a
/// delimiter-based string back into identity components and prevents
/// ambiguous concatenations.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QualifiedName {
    namespace: String,
    name: String,
}

impl QualifiedName {
    /// Construct a validated qualified identity.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> ExtensionValidationResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        validate_component(&namespace, "namespace")?;
        validate_component(&name, "name")?;

        Ok(Self { namespace, name })
    }

    /// Return the namespace component.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the local name component.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return a stable display representation.
    ///
    /// This representation is for display and diagnostics. Equality and
    /// ordering use the original components rather than reparsing this text.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }
}

impl fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.namespace, self.name)
    }
}

/// Stable identity of one extension definition.
///
/// This type is not a computational-domain identity or target identity.
pub type ExtensionId = QualifiedName;

/// Semantic version of an extension contract.
///
/// A version identifies the extension's declared semantic contract. It does
/// not by itself prove that the implementation conforms to that contract.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtensionVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl ExtensionVersion {
    /// Construct a semantic version.
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Return the major version.
    #[must_use]
    pub const fn major(self) -> u64 {
        self.major
    }

    /// Return the minor version.
    #[must_use]
    pub const fn minor(self) -> u64 {
        self.minor
    }

    /// Return the patch version.
    #[must_use]
    pub const fn patch(self) -> u64 {
        self.patch
    }
}

impl fmt::Display for ExtensionVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// A half-open version range.
///
/// `minimum_inclusive` is included when present. `maximum_exclusive` is
/// excluded when present. An absent bound means that the range is unbounded
/// in that direction.
///
/// An empty or contradictory range is rejected at construction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct VersionRange {
    minimum_inclusive: Option<ExtensionVersion>,
    maximum_exclusive: Option<ExtensionVersion>,
}

impl VersionRange {
    /// Construct a version range.
    ///
    /// # Errors
    ///
    /// Returns `ExtensionInputError::EmptyVersionRange` if both bounds exist
    /// and the lower bound is greater than or equal to the upper bound.
    pub fn new(
        minimum_inclusive: Option<ExtensionVersion>,
        maximum_exclusive: Option<ExtensionVersion>,
    ) -> ExtensionValidationResult<Self> {
        if let (Some(minimum), Some(maximum)) =
            (minimum_inclusive, maximum_exclusive)
        {
            if minimum >= maximum {
                return Err(ExtensionInputError::EmptyVersionRange {
                    minimum,
                    maximum,
                });
            }
        }

        Ok(Self {
            minimum_inclusive,
            maximum_exclusive,
        })
    }

    /// Accept every version.
    #[must_use]
    pub const fn any() -> Self {
        Self {
            minimum_inclusive: None,
            maximum_exclusive: None,
        }
    }

    /// Return the inclusive lower bound, if present.
    #[must_use]
    pub const fn minimum_inclusive(self) -> Option<ExtensionVersion> {
        self.minimum_inclusive
    }

    /// Return the exclusive upper bound, if present.
    #[must_use]
    pub const fn maximum_exclusive(self) -> Option<ExtensionVersion> {
        self.maximum_exclusive
    }

    /// Return whether the range accepts a version.
    #[must_use]
    pub fn contains(self, version: ExtensionVersion) -> bool {
        self.minimum_inclusive
            .is_none_or(|minimum| version >= minimum)
            && self
                .maximum_exclusive
                .is_none_or(|maximum| version < maximum)
    }
}

/// Classification of an extension's architectural role.
///
/// This is extensible at the metadata level: a new domain or vendor
/// contribution does not require adding a new enum variant. The enum
/// classifies broad lifecycle and trust categories, not every possible domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ExtensionKind {
    /// Part of the language implementation's core semantic contract.
    Core,

    /// Official, project-maintained extension.
    Standard,

    /// Official or registered computational-domain contribution.
    Domain,

    /// Experimental contribution that may change before stabilization.
    Experimental,

    /// Third-party extension not maintained as part of the core language.
    External,

    /// Vendor-scoped contribution that must remain namespace-isolated.
    Vendor,
}

impl ExtensionKind {
    /// Return whether this kind is part of the language core.
    #[must_use]
    pub const fn is_core(self) -> bool {
        matches!(self, Self::Core)
    }
}

/// Stability classification of an extension contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExtensionStability {
    /// Contract intended for stable use.
    Stable,

    /// Contract is experimental and may change incompatibly.
    Experimental,
}

/// Dependency on another extension and its accepted version range.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtensionDependency {
    id: ExtensionId,
    versions: VersionRange,
    optional: bool,
}

impl ExtensionDependency {
    /// Construct an extension dependency.
    #[must_use]
    pub const fn new(
        id: ExtensionId,
        versions: VersionRange,
        optional: bool,
    ) -> Self {
        Self {
            id,
            versions,
            optional,
        }
    }

    /// Return the required extension identity.
    #[must_use]
    pub fn id(&self) -> &ExtensionId {
        &self.id
    }

    /// Return the accepted dependency version range.
    #[must_use]
    pub const fn versions(&self) -> VersionRange {
        self.versions
    }

    /// Return whether the dependency is optional.
    #[must_use]
    pub const fn is_optional(&self) -> bool {
        self.optional
    }
}

/// One extension contribution declared by a manifest.
///
/// A contribution might identify a type family, effect, operation contract,
/// domain rule, dialect feature, or another semantic entity. This record does
/// not decide the contribution's meaning; the owning semantic subsystem does.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtensionContribution {
    id: QualifiedName,
    contract_version: ExtensionVersion,
}

impl ExtensionContribution {
    /// Construct a contribution identity and contract version.
    #[must_use]
    pub const fn new(
        id: QualifiedName,
        contract_version: ExtensionVersion,
    ) -> Self {
        Self {
            id,
            contract_version,
        }
    }

    /// Return the qualified contribution identity.
    #[must_use]
    pub fn id(&self) -> &QualifiedName {
        &self.id
    }

    /// Return the contribution's contract version.
    #[must_use]
    pub const fn contract_version(&self) -> ExtensionVersion {
        self.contract_version
    }
}

/// Manifest for one semantic extension.
///
/// Fields are private to preserve construction invariants. Use
/// [`ExtensionManifest::new`] and the explicit builder-style methods.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtensionManifest {
    id: ExtensionId,
    version: ExtensionVersion,
    kind: ExtensionKind,
    stability: ExtensionStability,
    description: String,
    language_versions: VersionRange,
    dependencies: Vec<ExtensionDependency>,
    conflicts: BTreeSet<ExtensionId>,
    contributions: Vec<ExtensionContribution>,
    required_capabilities: BTreeSet<QualifiedName>,
    source_node: Option<NodeId>,
}

impl ExtensionManifest {
    /// Construct a manifest with required identity, version, classification,
    /// stability, and human-readable description.
    ///
    /// # Errors
    ///
    /// Returns an error if the description is empty or whitespace-only.
    pub fn new(
        id: ExtensionId,
        version: ExtensionVersion,
        kind: ExtensionKind,
        stability: ExtensionStability,
        description: impl Into<String>,
    ) -> ExtensionValidationResult<Self> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(ExtensionInputError::EmptyDescription);
        }

        Ok(Self {
            id,
            version,
            kind,
            stability,
            description,
            language_versions: VersionRange::any(),
            dependencies: Vec::new(),
            conflicts: BTreeSet::new(),
            contributions: Vec::new(),
            required_capabilities: BTreeSet::new(),
            source_node: None,
        })
    }

    /// Restrict the language versions accepted by this extension.
    #[must_use]
    pub fn with_language_versions(mut self, versions: VersionRange) -> Self {
        self.language_versions = versions;
        self
    }

    /// Attach a declared dependency.
    ///
    /// Duplicate dependencies are reported during set validation so callers
    /// can collect all manifest problems in one pass.
    #[must_use]
    pub fn with_dependency(mut self, dependency: ExtensionDependency) -> Self {
        self.dependencies.push(dependency);
        self
    }

    /// Declare an extension that cannot be active at the same time.
    #[must_use]
    pub fn with_conflict(mut self, other: ExtensionId) -> Self {
        self.conflicts.insert(other);
        self
    }

    /// Declare a semantic contribution.
    #[must_use]
    pub fn with_contribution(
        mut self,
        contribution: ExtensionContribution,
    ) -> Self {
        self.contributions.push(contribution);
        self
    }

    /// Declare a capability requirement for the extension's semantic use.
    ///
    /// This records source-level metadata only. It does not prove that a
    /// target supplies the capability or authorize any runtime action.
    #[must_use]
    pub fn requiring_capability(mut self, capability: QualifiedName) -> Self {
        self.required_capabilities.insert(capability);
        self
    }

    /// Associate the manifest with its canonical AST node.
    #[must_use]
    pub const fn with_source_node(mut self, node: NodeId) -> Self {
        self.source_node = Some(node);
        self
    }

    /// Return the extension identity.
    #[must_use]
    pub fn id(&self) -> &ExtensionId {
        &self.id
    }

    /// Return the extension contract version.
    #[must_use]
    pub const fn version(&self) -> ExtensionVersion {
        self.version
    }

    /// Return the extension kind.
    #[must_use]
    pub const fn kind(&self) -> ExtensionKind {
        self.kind
    }

    /// Return the stability classification.
    #[must_use]
    pub const fn stability(&self) -> ExtensionStability {
        self.stability
    }

    /// Return the human-readable description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Return the supported language-version range.
    #[must_use]
    pub const fn language_versions(&self) -> VersionRange {
        self.language_versions
    }

    /// Return declared dependencies in declaration order.
    #[must_use]
    pub fn dependencies(&self) -> &[ExtensionDependency] {
        &self.dependencies
    }

    /// Return declared conflicts in stable identity order.
    #[must_use]
    pub fn conflicts(&self) -> &BTreeSet<ExtensionId> {
        &self.conflicts
    }

    /// Return declared semantic contributions in declaration order.
    #[must_use]
    pub fn contributions(&self) -> &[ExtensionContribution] {
        &self.contributions
    }

    /// Return required capabilities in stable identity order.
    #[must_use]
    pub fn required_capabilities(&self) -> &BTreeSet<QualifiedName> {
        &self.required_capabilities
    }

    /// Return the source AST node, if supplied by the adapter.
    #[must_use]
    pub const fn source_node(&self) -> Option<NodeId> {
        self.source_node
    }
}

/// Errors while constructing an extension identity, range, or manifest.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExtensionInputError {
    /// An identity component was empty.
    EmptyIdentityComponent {
        /// Identity component label.
        label: &'static str,
    },

    /// An identity component contained a control character.
    ControlCharacterInIdentity {
        /// Identity component label.
        label: &'static str,
    },

    /// An identity component contained whitespace.
    WhitespaceInIdentity {
        /// Identity component label.
        label: &'static str,
    },

    /// A version range was empty or contradictory.
    EmptyVersionRange {
        /// Inclusive lower bound.
        minimum: ExtensionVersion,
        /// Exclusive upper bound.
        maximum: ExtensionVersion,
    },

    /// A manifest description was empty.
    EmptyDescription,
}

impl fmt::Display for ExtensionInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentityComponent { label } => {
                write!(f, "extension {label} must not be empty")
            }
            Self::ControlCharacterInIdentity { label } => {
                write!(f, "extension {label} contains a control character")
            }
            Self::WhitespaceInIdentity { label } => {
                write!(f, "extension {label} contains whitespace")
            }
            Self::EmptyVersionRange { minimum, maximum } => {
                write!(
                    f,
                    "empty version range: lower bound {minimum} is not \
                     below upper bound {maximum}"
                )
            }
            Self::EmptyDescription => {
                f.write_str("extension description must not be empty")
            }
        }
    }
}

impl Error for ExtensionInputError {}

/// Severity of a manifest-validation issue.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExtensionIssueSeverity {
    /// The manifest set cannot be activated as declared.
    Error,

    /// The manifest set is usable, but has a noteworthy condition.
    Warning,
}

/// Stable category for extension-validation issues.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ExtensionIssueKind {
    /// The same extension identity appears more than once.
    DuplicateExtension,

    /// The active language version is outside the extension's supported range.
    LanguageVersionIncompatible,

    /// An extension is disallowed by the configured extension policy.
    PolicyViolation,

    /// A required dependency was not registered.
    MissingDependency,

    /// A dependency's registered version does not satisfy its range.
    DependencyVersionIncompatible,

    /// The same dependency was declared more than once.
    DuplicateDependency,

    /// A manifest conflicts with another registered extension.
    ExtensionConflict,

    /// Two extensions claim the same contribution identity.
    DuplicateContribution,

    /// A manifest depends on itself.
    SelfDependency,

    /// The extension dependency graph contains a cycle.
    DependencyCycle,

    /// A manifest is internally inconsistent.
    InvalidManifest,
}

impl fmt::Display for ExtensionIssueKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::DuplicateExtension => "duplicate extension",
            Self::LanguageVersionIncompatible => "language-version incompatibility",
            Self::PolicyViolation => "extension-policy violation",
            Self::MissingDependency => "missing dependency",
            Self::DependencyVersionIncompatible => "dependency-version incompatibility",
            Self::DuplicateDependency => "duplicate dependency",
            Self::ExtensionConflict => "extension conflict",
            Self::DuplicateContribution => "duplicate contribution",
            Self::SelfDependency => "self-dependency",
            Self::DependencyCycle => "dependency cycle",
            Self::InvalidManifest => "invalid extension manifest",
        };

        f.write_str(label)
    }
}

/// One structured issue discovered while validating extension manifests.
///
/// Consumers must classify issues using `kind` and `severity`, not by parsing
/// the human-readable message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtensionIssue {
    /// Stable issue category.
    pub kind: ExtensionIssueKind,

    /// Severity of the issue.
    pub severity: ExtensionIssueSeverity,

    /// Primary extension involved, if known.
    pub extension: Option<ExtensionId>,

    /// Related extension involved, if known.
    pub related_extension: Option<ExtensionId>,

    /// Source AST node associated with the issue, if known.
    pub source_node: Option<NodeId>,

    /// Human-readable explanation.
    pub message: String,
}

impl ExtensionIssue {
    fn new(
        kind: ExtensionIssueKind,
        extension: Option<ExtensionId>,
        related_extension: Option<ExtensionId>,
        source_node: Option<NodeId>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            severity: ExtensionIssueSeverity::Error,
            extension,
            related_extension,
            source_node,
            message: message.into(),
        }
    }
}

/// Result of validating a set of extension manifests.
///
/// `dependency_order` is only an ordering plan for metadata activation. It
/// does not load or execute extension code. If the manifest set contains
/// errors, callers must not treat the order as permission to activate it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtensionValidationReport {
    issues: Vec<ExtensionIssue>,
    dependency_order: Vec<ExtensionId>,
}

impl ExtensionValidationReport {
    /// Return issues in deterministic order.
    #[must_use]
    pub fn issues(&self) -> &[ExtensionIssue] {
        &self.issues
    }

    /// Return the dependency-first ordering when the dependency graph is
    /// acyclic. The list is empty if a cycle prevents a complete ordering.
    #[must_use]
    pub fn dependency_order(&self) -> &[ExtensionId] {
        &self.dependency_order
    }

    /// Return whether the manifest set has no errors.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self
            .issues
            .iter()
            .any(|issue| issue.severity == ExtensionIssueSeverity::Error)
    }

    /// Return whether at least one issue was reported.
    #[must_use]
    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }
}

/// Validate a collection of extension manifests.
///
/// The input order does not determine dependency order. Duplicate identities
/// are diagnosed and the first occurrence is retained for follow-on checks,
/// allowing one invocation to report multiple independent issues.
///
/// `language_version` must be the semantic language version selected for the
/// current analysis, not the compiler crate version or target version.
///
/// The caller must not activate extensions if `report.is_valid()` is false.
#[must_use]
pub fn validate_extensions(
    manifests: &[ExtensionManifest],
    language_version: ExtensionVersion,
    policy: ExtensionPolicy,
) -> ExtensionValidationReport {
    let mut issues = Vec::new();
    let mut by_id: BTreeMap<ExtensionId, &ExtensionManifest> = BTreeMap::new();

    // Index manifests and report duplicate identities deterministically.
    for manifest in manifests {
        if let Some(previous) = by_id.get(manifest.id()) {
            issues.push(ExtensionIssue::new(
                ExtensionIssueKind::DuplicateExtension,
                Some(manifest.id().clone()),
                Some(previous.id().clone()),
                manifest.source_node(),
                format!(
                    "extension `{}` is declared more than once",
                    manifest.id()
                ),
            ));
        } else {
            by_id.insert(manifest.id().clone(), manifest);
        }
    }

    let mut contribution_owners: BTreeMap<QualifiedName, ExtensionId> =
        BTreeMap::new();

    for manifest in by_id.values().copied() {
        if !manifest.language_versions().contains(language_version) {
            issues.push(ExtensionIssue::new(
                ExtensionIssueKind::LanguageVersionIncompatible,
                Some(manifest.id().clone()),
                None,
                manifest.source_node(),
                format!(
                    "extension `{}` version {} does not support language version {}",
                    manifest.id(),
                    manifest.version(),
                    language_version
                ),
            ));
        }

        match policy {
            ExtensionPolicy::Disabled if !manifest.kind().is_core() => {
                issues.push(ExtensionIssue::new(
                    ExtensionIssueKind::PolicyViolation,
                    Some(manifest.id().clone()),
                    None,
                    manifest.source_node(),
                    format!(
                        "extension `{}` is present while semantic extensions are disabled",
                        manifest.id()
                    ),
                ));
            }
            ExtensionPolicy::StableOnly
                if manifest.stability() == ExtensionStability::Experimental
                    || manifest.kind() == ExtensionKind::Experimental =>
            {
                issues.push(ExtensionIssue::new(
                    ExtensionIssueKind::PolicyViolation,
                    Some(manifest.id().clone()),
                    None,
                    manifest.source_node(),
                    format!(
                        "extension `{}` is experimental and is not allowed by stable-only policy",
                        manifest.id()
                    ),
                ));
            }
            ExtensionPolicy::StableOnly
            | ExtensionPolicy::AllowExperimental
            | ExtensionPolicy::Disabled => {}
        }

        let mut seen_dependencies = BTreeSet::new();

        for dependency in manifest.dependencies() {
            if !seen_dependencies.insert(dependency.id().clone()) {
                issues.push(ExtensionIssue::new(
                    ExtensionIssueKind::DuplicateDependency,
                    Some(manifest.id().clone()),
                    Some(dependency.id().clone()),
                    manifest.source_node(),
                    format!(
                        "extension `{}` declares dependency `{}` more than once",
                        manifest.id(),
                        dependency.id()
                    ),
                ));
                continue;
            }

            if dependency.id() == manifest.id() {
                issues.push(ExtensionIssue::new(
                    ExtensionIssueKind::SelfDependency,
                    Some(manifest.id().clone()),
                    Some(dependency.id().clone()),
                    manifest.source_node(),
                    format!(
                        "extension `{}` cannot depend on itself",
                        manifest.id()
                    ),
                ));
                continue;
            }

            match by_id.get(dependency.id()) {
                None if !dependency.is_optional() => {
                    issues.push(ExtensionIssue::new(
                        ExtensionIssueKind::MissingDependency,
                        Some(manifest.id().clone()),
                        Some(dependency.id().clone()),
                        manifest.source_node(),
                        format!(
                            "extension `{}` requires missing dependency `{}`",
                            manifest.id(),
                            dependency.id()
                        ),
                    ));
                }
                None => {
                    // Optional absent dependencies do not create graph edges.
                }
                Some(dependency_manifest)
                    if !dependency.versions().contains(dependency_manifest.version()) =>
                {
                    issues.push(ExtensionIssue::new(
                        ExtensionIssueKind::DependencyVersionIncompatible,
                        Some(manifest.id().clone()),
                        Some(dependency.id().clone()),
                        manifest.source_node(),
                        format!(
                            "extension `{}` accepts dependency `{}` only in its declared version range; registered version is {}",
                            manifest.id(),
                            dependency.id(),
                            dependency_manifest.version()
                        ),
                    ));
                }
                Some(_) => {}
            }
        }

        for conflict in manifest.conflicts() {
            if by_id.contains_key(conflict) {
                issues.push(ExtensionIssue::new(
                    ExtensionIssueKind::ExtensionConflict,
                    Some(manifest.id().clone()),
                    Some(conflict.clone()),
                    manifest.source_node(),
                    format!(
                        "extension `{}` conflicts with registered extension `{}`",
                        manifest.id(),
                        conflict
                    ),
                ));
            }
        }

        for contribution in manifest.contributions() {
            let contribution_id = contribution.id().clone();

            if let Some(owner) = contribution_owners.get(&contribution_id) {
                if owner != manifest.id() {
                    issues.push(ExtensionIssue::new(
                        ExtensionIssueKind::DuplicateContribution,
                        Some(manifest.id().clone()),
                        Some(owner.clone()),
                        manifest.source_node(),
                        format!(
                            "contribution `{}` is claimed by both `{}` and `{}`",
                            contribution_id,
                            owner,
                            manifest.id()
                        ),
                    ));
                }
            } else {
                contribution_owners
                    .insert(contribution_id, manifest.id().clone());
            }
        }
    }

    let dependency_order = compute_dependency_order(&by_id, &mut issues);

    // Stable ordering independent of hash iteration or input ordering.
    issues.sort_by(|left, right| {
        left.extension
            .cmp(&right.extension)
            .then_with(|| left.kind.cmp(&right.kind))
            .then_with(|| left.related_extension.cmp(&right.related_extension))
            .then_with(|| left.source_node.cmp(&right.source_node))
            .then_with(|| left.message.cmp(&right.message))
    });

    ExtensionValidationReport {
        issues,
        dependency_order,
    }
}

/// Compute a deterministic dependency-first order using Kahn's algorithm.
///
/// The graph contains edges from a dependency to its dependent. Missing
/// optional dependencies are ignored. Missing required dependencies are
/// already reported by the caller and are not represented as graph nodes.
///
/// If a cycle exists, the function emits one deterministic issue for every
/// remaining node in the cyclic or cycle-blocked subgraph and returns an
/// empty ordering rather than a misleading partial activation plan.
fn compute_dependency_order(
    by_id: &BTreeMap<ExtensionId, &ExtensionManifest>,
    issues: &mut Vec<ExtensionIssue>,
) -> Vec<ExtensionId> {
    let mut indegree: BTreeMap<ExtensionId, usize> =
        by_id.keys().cloned().map(|id| (id, 0)).collect();

    let mut outgoing: BTreeMap<ExtensionId, BTreeSet<ExtensionId>> =
        by_id.keys().cloned().map(|id| (id, BTreeSet::new())).collect();

    for manifest in by_id.values().copied() {
        for dependency in manifest.dependencies() {
            if !by_id.contains_key(dependency.id()) {
                continue;
            }

            // Duplicate edges must not inflate indegree. Duplicate declarations
            // are diagnosed separately during manifest validation.
            let inserted = outgoing
                .entry(dependency.id().clone())
                .or_default()
                .insert(manifest.id().clone());

            if inserted {
                if let Some(degree) = indegree.get_mut(manifest.id()) {
                    *degree = degree.saturating_add(1);
                }
            }
        }
    }

    let mut ready: BTreeSet<ExtensionId> = indegree
        .iter()
        .filter_map(|(id, degree)| (*degree == 0).then_some(id.clone()))
        .collect();

    let mut ordered = Vec::with_capacity(by_id.len());

    while let Some(id) = ready.pop_first() {
        ordered.push(id.clone());

        if let Some(dependents) = outgoing.get(&id) {
            for dependent in dependents {
                if let Some(degree) = indegree.get_mut(dependent) {
                    *degree = degree.saturating_sub(1);

                    if *degree == 0 {
                        ready.insert(dependent.clone());
                    }
                }
            }
        }
    }

    if ordered.len() == by_id.len() {
        return ordered;
    }

    // All remaining nodes are part of a cycle or are blocked downstream
    // from one. Report each identity so diagnostics remain useful even when
    // the graph contains several independent cycles.
    for (id, degree) in &indegree {
        if *degree > 0 {
            let source_node = by_id.get(id).and_then(|manifest| manifest.source_node());

            issues.push(ExtensionIssue::new(
                ExtensionIssueKind::DependencyCycle,
                Some(id.clone()),
                None,
                source_node,
                format!(
                    "extension `{id}` participates in or is blocked by a dependency cycle"
                ),
            ));
        }
    }

    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(namespace: &str, name: &str) -> ExtensionId {
        QualifiedName::new(namespace, name).expect("test identity must be valid")
    }

    fn manifest(
        namespace: &str,
        name: &str,
        version: ExtensionVersion,
    ) -> ExtensionManifest {
        ExtensionManifest::new(
            id(namespace, name),
            version,
            ExtensionKind::Domain,
            ExtensionStability::Stable,
            "test extension",
        )
        .expect("test manifest must be valid")
    }

    fn language_version() -> ExtensionVersion {
        ExtensionVersion::new(1, 0, 0)
    }

    #[test]
    fn rejects_empty_identity_components() {
        assert!(QualifiedName::new("", "extension").is_err());
        assert!(QualifiedName::new("example", " ").is_err());
    }

    #[test]
    fn version_range_is_half_open() {
        let range = VersionRange::new(
            Some(ExtensionVersion::new(1, 0, 0)),
            Some(ExtensionVersion::new(2, 0, 0)),
        )
        .expect("range must be valid");

        assert!(range.contains(ExtensionVersion::new(1, 0, 0)));
        assert!(range.contains(ExtensionVersion::new(1, 9, 9)));
        assert!(!range.contains(ExtensionVersion::new(2, 0, 0)));
        assert!(!range.contains(ExtensionVersion::new(0, 9, 9)));
    }

    #[test]
    fn rejects_contradictory_version_range() {
        assert!(
            VersionRange::new(
                Some(ExtensionVersion::new(2, 0, 0)),
                Some(ExtensionVersion::new(2, 0, 0)),
            )
            .is_err()
        );
    }

    #[test]
    fn accepts_valid_dependency_order() {
        let base = manifest(
            "example",
            "base",
            ExtensionVersion::new(1, 0, 0),
        );

        let consumer = manifest(
            "example",
            "consumer",
            ExtensionVersion::new(1, 0, 0),
        )
        .with_dependency(ExtensionDependency::new(
            id("example", "base"),
            VersionRange::any(),
            false,
        ));

        let report = validate_extensions(
            &[consumer, base],
            language_version(),
            ExtensionPolicy::StableOnly,
        );

        assert!(report.is_valid(), "{:?}", report.issues());
        assert_eq!(
            report.dependency_order(),
            &[id("example", "base"), id("example", "consumer")]
        );
    }

    #[test]
    fn reports_missing_required_dependency() {
        let extension = manifest(
            "example",
            "consumer",
            ExtensionVersion::new(1, 0, 0),
        )
        .with_dependency(ExtensionDependency::new(
            id("example", "missing"),
            VersionRange::any(),
            false,
        ));

        let report = validate_extensions(
            &[extension],
            language_version(),
            ExtensionPolicy::StableOnly,
        );

        assert!(!report.is_valid());
        assert!(report
            .issues()
            .iter()
            .any(|issue| issue.kind == ExtensionIssueKind::MissingDependency));
    }

    #[test]
    fn reports_dependency_version_mismatch() {
        let base = manifest(
            "example",
            "base",
            ExtensionVersion::new(3, 0, 0),
        );

        let consumer = manifest(
            "example",
            "consumer",
            ExtensionVersion::new(1, 0, 0),
        )
        .with_dependency(ExtensionDependency::new(
            id("example", "base"),
            VersionRange::new(
                Some(ExtensionVersion::new(1, 0, 0)),
                Some(ExtensionVersion::new(2, 0, 0)),
            )
            .expect("range must be valid"),
            false,
        ));

        let report = validate_extensions(
            &[base, consumer],
            language_version(),
            ExtensionPolicy::StableOnly,
        );

        assert!(report.issues().iter().any(|issue| {
            issue.kind == ExtensionIssueKind::DependencyVersionIncompatible
        }));
    }

    #[test]
    fn reports_cycles_without_returning_partial_order() {
        let first = manifest(
            "example",
            "first",
            ExtensionVersion::new(1, 0, 0),
        )
        .with_dependency(ExtensionDependency::new(
            id("example", "second"),
            VersionRange::any(),
            false,
        ));

        let second = manifest(
            "example",
            "second",
            ExtensionVersion::new(1, 0, 0),
        )
        .with_dependency(ExtensionDependency::new(
            id("example", "first"),
            VersionRange::any(),
            false,
        ));

        let report = validate_extensions(
            &[first, second],
            language_version(),
            ExtensionPolicy::StableOnly,
        );

        assert!(!report.is_valid());
        assert!(report.dependency_order().is_empty());
        assert!(report
            .issues()
            .iter()
            .any(|issue| issue.kind == ExtensionIssueKind::DependencyCycle));
    }

    #[test]
    fn reports_duplicate_contributions_across_extensions() {
        let contribution = ExtensionContribution::new(
            id("example.api", "operation"),
            ExtensionVersion::new(1, 0, 0),
        );

        let first = manifest(
            "example",
            "first",
            ExtensionVersion::new(1, 0, 0),
        )
        .with_contribution(contribution.clone());

        let second = manifest(
            "example",
            "second",
            ExtensionVersion::new(1, 0, 0),
        )
        .with_contribution(contribution);

        let report = validate_extensions(
            &[first, second],
            language_version(),
            ExtensionPolicy::StableOnly,
        );

        assert!(report.issues().iter().any(|issue| {
            issue.kind == ExtensionIssueKind::DuplicateContribution
        }));
    }

    #[test]
    fn stable_only_policy_rejects_experimental_extensions() {
        let experimental = ExtensionManifest::new(
            id("example", "preview"),
            ExtensionVersion::new(0, 1, 0),
            ExtensionKind::Experimental,
            ExtensionStability::Experimental,
            "experimental test extension",
        )
        .expect("test manifest must be valid");

        let report = validate_extensions(
            &[experimental],
            language_version(),
            ExtensionPolicy::StableOnly,
        );

        assert!(report.issues().iter().any(|issue| {
            issue.kind == ExtensionIssueKind::PolicyViolation
        }));
    }

    #[test]
    fn disabled_policy_still_allows_core_definitions() {
        let core = ExtensionManifest::new(
            id("zamani.core", "standard"),
            ExtensionVersion::new(1, 0, 0),
            ExtensionKind::Core,
            ExtensionStability::Stable,
            "core test definition",
        )
        .expect("test manifest must be valid");

        let report = validate_extensions(
            &[core],
            language_version(),
            ExtensionPolicy::Disabled,
        );

        assert!(report.is_valid(), "{:?}", report.issues());
    }

    #[test]
    fn rejects_duplicate_extension_identities() {
        let first = manifest(
            "example",
            "duplicate",
            ExtensionVersion::new(1, 0, 0),
        );

        let second = manifest(
            "example",
            "duplicate",
            ExtensionVersion::new(2, 0, 0),
        );

        let report = validate_extensions(
            &[first, second],
            language_version(),
            ExtensionPolicy::StableOnly,
        );

        assert!(report.issues().iter().any(|issue| {
            issue.kind == ExtensionIssueKind::DuplicateExtension
        }));
    }

    #[test]
    fn diagnostic_order_does_not_depend_on_manifest_input_order() {
        let first = manifest(
            "example",
            "first",
            ExtensionVersion::new(1, 0, 0),
        )
        .with_dependency(ExtensionDependency::new(
            id("example", "missing_a"),
            VersionRange::any(),
            false,
        ));

        let second = manifest(
            "example",
            "second",
            ExtensionVersion::new(1, 0, 0),
        )
        .with_dependency(ExtensionDependency::new(
            id("example", "missing_b"),
            VersionRange::any(),
            false,
        ));

        let left = validate_extensions(
            &[first.clone(), second.clone()],
            language_version(),
            ExtensionPolicy::StableOnly,
        );

        let right = validate_extensions(
            &[second, first],
            language_version(),
            ExtensionPolicy::StableOnly,
        );

        assert_eq!(left.issues(), right.issues());
    }
}