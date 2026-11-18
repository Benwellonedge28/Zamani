
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Documentation Command
//!
//! Path: `src/toolchain/commands/doc.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the toolchain-facing documentation-generation command.
//!
//! The command coordinates an explicitly supplied documentation provider,
//! validates the request and returned artifacts, requires provider
//! verification, and returns a structured report.
//!
//! ## Ownership
//!
//! This module owns:
//! - Documentation request and option contracts.
//! - Documentation scope identifiers.
//! - Documentation provider identity and integration contract.
//! - Documentation-generation orchestration.
//! - Documentation output completeness requirements.
//! - Documentation artifact validation.
//! - Structured documentation operation results.
//!
//! This module does not own:
//! - Zamani grammar, lexical analysis, parsing, or AST construction.
//! - Documentation-comment recognition or declaration association.
//! - Semantic analysis or source-language meaning.
//! - Rust API extraction or Rustdoc implementation.
//! - Markdown, HTML, or other documentation rendering engines.
//! - Documentation templates or theme systems.
//! - Filesystem traversal, directory creation, or file replacement.
//! - Artifact storage implementation.
//! - Compiler execution, backend generation, hardware discovery,
//!   target selection, or runtime execution.
//!
//! ## Canonical integration authorities
//!
//! - `crate::toolchain::errors` owns toolchain errors.
//! - `crate::toolchain::result` re-exports the canonical result contract.
//! - `crate::compiler::artifacts::artifact` owns artifact representation.
//! - `crate::frontend::lexer` owns canonical lexical processing.
//! - `crate::frontend::ast` owns canonical AST structures and metadata.
//! - `crate::frontend::semantic` owns source-level semantic meaning.
//! - `crate::compiler` owns compiler-facing orchestration.
//! - `grammar/` owns normative Zamani syntax and documentation-comment rules.
//! - `src/compiler/documentation/` contains documentation architecture
//!   and integration specifications.
//!
//! A concrete provider must integrate with these authorities. This module
//! deliberately does not assume that a documentation generator already
//! exists.
//!
//! ## Documentation sources
//!
//! Providers may generate documentation from:
//! - Zamani project metadata and manifests.
//! - Canonical Zamani source and AST structures.
//! - Canonically associated documentation comments.
//! - Public API and declaration metadata.
//! - Compiler-produced metadata and verified artifacts.
//! - Rust API metadata through an explicitly supported Rust documentation
//!   provider.
//! - Registered domain-specific documentation metadata.
//!
//! A provider must not create a competing Zamani parser, grammar, AST,
//! semantic model, or documentation-comment recognizer.
//!
//! ## POCO-REAF
//!
//! Documentation generation is independent of physical target realization.
//!
//! The command accepts open-ended domain identifiers. It does not maintain
//! a closed list of classical, quantum, HDL, AI, hybrid, distributed,
//! networking, cryptographic, embedded, accelerator, or future domains.
//!
//! Documentation must describe the actual capabilities and limitations
//! established by authoritative providers. It must not claim that a
//! target, backend, runtime, or computational domain is implemented merely
//! because a name appears in documentation metadata.
//!
//! ## Scalability
//!
//! This module imposes no arbitrary maximum on:
//! - Documentation sources.
//! - Source files.
//! - Declarations.
//! - Documentation comments.
//! - API items.
//! - Domains.
//! - Output formats.
//! - Generated artifacts.
//! - Providers.
//!
//! Collections grow subject to available memory and address space.
//! Providers may use streaming and external artifact storage for large
//! documentation products.
//!
//! Operational limits must be explicit and configurable through the
//! owning configuration and provider services. No language-wide or
//! hardware-capacity limit is introduced here.
//!
//! Literal infinite storage or generation is not promised.
//!
//! ## Determinism
//!
//! For identical project inputs, language version, provider version,
//! configuration, and source metadata, providers should produce
//! reproducible output.
//!
//! Providers must not introduce nondeterministic ordering from hash-map
//! iteration, wall-clock time, random state, host hardware, or network
//! availability unless the output explicitly records that dependency.
//!
//! ## Security
//!
//! Documentation comments and project metadata are untrusted input.
//! Providers must not execute directives, shell fragments, scripts,
//! macros, or embedded commands merely because they appear in comments.
//!
//! Providers writing to a destination must validate paths, reject unsafe
//! path traversal, handle symlinks according to policy, and use safe
//! replacement semantics. They must not overwrite unrelated user files.
//!
//! Credentials and secret configuration must never be embedded in
//! generated documentation or operation reports.
//!
//! ## Failure semantics
//!
//! A successful report requires:
//! 1. A structurally valid request.
//! 2. A provider that supports the requested scope and configuration.
//! 3. Successful generation.
//! 4. At least one canonical documentation artifact.
//! 5. Documentation artifact kind validation.
//! 6. Inline artifact digest verification.
//! 7. Successful provider verification of the complete generation.
//!
//! External artifact payloads must be verified by the provider using the
//! canonical artifact-storage authority. A reference alone does not prove
//! that external content is available or verified.
//!
//! Any failure remains an error. Incomplete output must never be reported
//! as a completed documentation operation.
//!
//! ## Filesystem boundary
//!
//! This command does not traverse source directories or write files.
//! A provider may use the requested destination, but it owns the secure
//! filesystem and artifact-storage implementation.
//!
//! The CLI and API must invoke this same command service rather than
//! implementing independent documentation-generation behavior.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use crate::compiler::artifacts::artifact::Artifact;
use crate::toolchain::errors::{Result, ToolchainError};

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// Stable identity of a documentation provider.
///
/// This is an open-ended identity, not a closed list of documentation
/// engines or vendors.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DocumentationProviderId(String);

impl DocumentationProviderId {
    /// Creates a validated provider identity.
    ///
    /// # Errors
    ///
    /// Returns an error when the identity is empty, contains only
    /// whitespace, has surrounding whitespace, or contains control
    /// characters.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        validate_identifier("documentation provider identity", &value)?;

        Ok(Self(value))
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Identifies the documentation scope requested by the caller.
///
/// `Custom` permits future documentation scopes without changing this
/// command's public API.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DocumentationScope {
    /// Documentation for the Zamani project as a whole.
    Project,

    /// Public or configured API documentation.
    Api,

    /// Documentation extracted from Zamani source declarations.
    Source,

    /// All documentation scopes supported by the selected provider.
    All,

    /// A provider-defined documentation scope.
    Custom(String),
}

impl DocumentationScope {
    /// Validates the scope identifier.
    ///
    /// # Errors
    ///
    /// Returns an error when a custom scope is invalid.
    pub fn validate(&self) -> Result<()> {
        if let Self::Custom(value) = self {
            validate_identifier("documentation scope", value)?;
        }

        Ok(())
    }

    /// Returns the stable built-in or custom scope identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Project => "project",
            Self::Api => "api",
            Self::Source => "source",
            Self::All => "all",
            Self::Custom(value) => value,
        }
    }
}

// ============================================================================
// OPTIONS
// ============================================================================

/// Provider-neutral documentation-generation options.
///
/// Provider-specific options use namespaced keys. This type deliberately
/// does not prescribe renderer-specific configuration.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DocumentationOptions {
    /// Whether private or non-exported declarations should be included.
    pub include_private_items: bool,

    /// Whether documentation should include dependency API information.
    pub include_dependencies: bool,

    /// Optional Zamani language version or edition.
    pub language_version: Option<String>,

    /// Open-ended computational-domain identifiers.
    ///
    /// Examples include `classical`, `quantum`, `hdl`, `ai`, `hybrid`,
    /// and `distributed`. These are identifiers, not a closed enum.
    pub domains: BTreeSet<String>,

    /// Requested output-format identifiers.
    ///
    /// Examples include `html`, `markdown`, and `json`. Actual support
    /// must be established by the selected provider.
    pub formats: BTreeSet<String>,

    /// Deterministically ordered provider-specific settings.
    pub settings: BTreeMap<String, String>,
}

impl DocumentationOptions {
    /// Creates default documentation options.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Includes private or non-exported declarations.
    #[must_use]
    pub fn with_private_items(mut self, enabled: bool) -> Self {
        self.include_private_items = enabled;
        self
    }

    /// Includes dependency API information.
    #[must_use]
    pub fn with_dependencies(mut self, enabled: bool) -> Self {
        self.include_dependencies = enabled;
        self
    }

    /// Sets the language version or edition.
    #[must_use]
    pub fn with_language_version(
        mut self,
        version: impl Into<String>,
    ) -> Self {
        self.language_version = Some(version.into());
        self
    }

    /// Adds an open-ended computational-domain identifier.
    #[must_use]
    pub fn with_domain(mut self, domain: impl Into<String>) -> Self {
        self.domains.insert(domain.into());
        self
    }

    /// Adds an output-format identifier.
    #[must_use]
    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.formats.insert(format.into());
        self
    }

    /// Adds or replaces a provider-specific setting.
    #[must_use]
    pub fn with_setting(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.settings.insert(key.into(), value.into());
        self
    }

    /// Validates option metadata without interpreting provider-specific
    /// settings.
    ///
    /// # Errors
    ///
    /// Returns an error when an identifier or setting key is invalid.
    pub fn validate(&self) -> Result<()> {
        if let Some(version) = &self.language_version {
            validate_identifier("language version", version)?;
        }

        for domain in &self.domains {
            validate_identifier("documentation domain", domain)?;
        }

        for format in &self.formats {
            validate_identifier("documentation format", format)?;
        }

        for key in self.settings.keys() {
            validate_identifier("documentation setting key", key)?;
        }

        Ok(())
    }
}

// ============================================================================
// REQUEST
// ============================================================================

/// Request to generate project, source, or API documentation.
///
/// Paths are references supplied by the caller. This request constructor
/// does not access the filesystem or claim that the paths exist.
///
/// The provider is responsible for canonical workspace resolution,
/// filesystem authorization, source discovery, and safe output handling.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentationRequest {
    /// Project or workspace root.
    pub project_root: PathBuf,

    /// Optional output destination.
    ///
    /// When absent, the provider may use its configured artifact store.
    pub destination: Option<PathBuf>,

    /// Requested documentation scope.
    pub scope: DocumentationScope,

    /// Documentation options.
    pub options: DocumentationOptions,
}

impl DocumentationRequest {
    /// Creates a documentation request.
    #[must_use]
    pub fn new(project_root: impl Into<PathBuf>) -> Self {
        Self {
            project_root: project_root.into(),
            destination: None,
            scope: DocumentationScope::Project,
            options: DocumentationOptions::default(),
        }
    }

    /// Sets the output destination.
    #[must_use]
    pub fn with_destination(
        mut self,
        destination: impl Into<PathBuf>,
    ) -> Self {
        self.destination = Some(destination.into());
        self
    }

    /// Sets the documentation scope.
    #[must_use]
    pub fn with_scope(mut self, scope: DocumentationScope) -> Self {
        self.scope = scope;
        self
    }

    /// Sets documentation options.
    #[must_use]
    pub fn with_options(mut self, options: DocumentationOptions) -> Self {
        self.options = options;
        self
    }

    /// Validates request structure.
    ///
    /// This method does not check filesystem existence, permissions,
    /// symlinks, project validity, manifest compatibility, or provider
    /// support. Those checks belong to the corresponding authorities.
    ///
    /// # Errors
    ///
    /// Returns an error for empty paths or invalid scope/options.
    pub fn validate(&self) -> Result<()> {
        validate_path("project root", &self.project_root)?;

        if let Some(destination) = &self.destination {
            validate_path("documentation destination", destination)?;
        }

        self.scope.validate()?;
        self.options.validate()?;

        Ok(())
    }
}

// ============================================================================
// PROVIDER OUTPUT
// ============================================================================

/// A complete set of documentation artifacts returned by a provider.
///
/// Artifacts use the canonical compiler artifact model. This module does
/// not create a competing documentation artifact or storage format.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentationGeneration {
    artifacts: Vec<Artifact>,
}

impl DocumentationGeneration {
    /// Creates a documentation generation from canonical artifacts.
    ///
    /// Every artifact must have the canonical `documentation` artifact
    /// kind. Inline payloads are verified immediately. External payloads
    /// must be verified by the provider during `verify`.
    ///
    /// # Errors
    ///
    /// Returns an error when the output is empty, contains an artifact
    /// of another kind, or has an invalid inline payload digest.
    pub fn new(artifacts: Vec<Artifact>) -> Result<Self> {
        if artifacts.is_empty() {
            return Err(ToolchainError::provider_failed(
                "documentation provider returned no documentation artifacts",
            )
            .with_operation("doc.output.validate"));
        }

        for artifact in &artifacts {
            if artifact.kind().as_str() != "documentation" {
                return Err(ToolchainError::provider_failed(format!(
                    "artifact '{}' has kind '{}', expected 'documentation'",
                    artifact.id(),
                    artifact.kind().as_str(),
                ))
                .with_operation("doc.output.validate"));
            }

            if artifact.has_inline_payload() {
                artifact
                    .verify_inline_payload()
                    .map_err(|error| {
                        ToolchainError::provider_failed(
                            "documentation artifact payload integrity verification failed",
                        )
                        .with_operation("doc.output.integrity")
                        .with_subject(artifact.id().to_string())
                        .with_source(error)
                    })?;
            }
        }

        Ok(Self { artifacts })
    }

    /// Returns the generated canonical artifacts.
    #[must_use]
    pub fn artifacts(&self) -> &[Artifact] {
        &self.artifacts
    }

    /// Returns the number of generated artifacts.
    #[must_use]
    pub fn artifact_count(&self) -> usize {
        self.artifacts.len()
    }

    /// Consumes the generation and returns its artifacts.
    #[must_use]
    pub fn into_artifacts(self) -> Vec<Artifact> {
        self.artifacts
    }
}

// ============================================================================
// PROVIDER CONTRACT
// ============================================================================

/// Documentation-generation provider contract.
///
/// Implementations may adapt a native Zamani documentation generator,
/// Rust API documentation generation, or another compatible documentation
/// engine.
///
/// A provider must use the canonical source, AST, semantic, manifest, and
/// artifact authorities as appropriate to the requested scope.
///
/// It must not create a competing language parser or reinterpret comments.
pub trait DocumentationProvider: Send + Sync {
    /// Returns the stable provider identity.
    fn id(&self) -> &DocumentationProviderId;

    /// Determines whether this provider supports the requested scope,
    /// language version, domains, formats, and options.
    ///
    /// `Ok(false)` means the provider does not support the request.
    /// An error means support could not be established reliably.
    fn supports(&self, request: &DocumentationRequest) -> Result<bool>;

    /// Generates documentation using the supplied request.
    ///
    /// The returned artifacts must represent actual generated output.
    /// A provider must not return placeholder artifacts or claim that
    /// unsupported source constructs were documented.
    ///
    /// # Errors
    ///
    /// Returns a structured error when discovery, extraction, rendering,
    /// artifact creation, storage, or another generation stage fails.
    fn generate(
        &self,
        request: &DocumentationRequest,
    ) -> Result<DocumentationGeneration>;

    /// Verifies that the generation is complete and corresponds to the
    /// supplied request.
    ///
    /// Implementations must verify external artifact payloads through the
    /// canonical artifact-storage provider, validate output completeness,
    /// and ensure that the requested destination or storage operation
    /// completed according to its contract.
    ///
    /// A provider must return an error if any required source, declaration,
    /// output format, or artifact was omitted.
    ///
    /// # Errors
    ///
    /// Returns an error when completeness, provenance, destination
    /// integrity, or external payload verification cannot be established.
    fn verify(
        &self,
        request: &DocumentationRequest,
        generation: &DocumentationGeneration,
    ) -> Result<()>;
}

// ============================================================================
// REPORT
// ============================================================================

/// Structured result of a successful documentation operation.
///
/// A report can only be constructed by `generate_documentation` after
/// generation and provider verification have completed successfully.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentationReport {
    provider: DocumentationProviderId,
    scope: DocumentationScope,
    artifacts: Vec<Artifact>,
    verified: bool,
}

impl DocumentationReport {
    /// Returns the provider that generated the documentation.
    #[must_use]
    pub fn provider(&self) -> &DocumentationProviderId {
        &self.provider
    }

    /// Returns the requested documentation scope.
    #[must_use]
    pub fn scope(&self) -> &DocumentationScope {
        &self.scope
    }

    /// Returns the generated canonical artifacts.
    #[must_use]
    pub fn artifacts(&self) -> &[Artifact] {
        &self.artifacts
    }

    /// Returns the number of generated artifacts.
    #[must_use]
    pub fn artifact_count(&self) -> usize {
        self.artifacts.len()
    }

    /// Returns whether provider verification completed successfully.
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.verified
    }
}

// ============================================================================
// COMMAND
// ============================================================================

/// Generates documentation using an explicitly supplied provider.
///
/// This is the principal toolchain documentation-command entry point.
///
/// Provider discovery and registration belong to the canonical provider
/// registry. This function does not instantiate an implicit default
/// provider or silently select an arbitrary documentation engine.
///
/// It does not:
/// - Parse Zamani source independently.
/// - Extract documentation comments itself.
/// - Generate placeholder output.
/// - Execute comment directives.
/// - Execute shell commands.
/// - Traverse directories itself.
/// - Write or overwrite files itself.
/// - Claim completeness without provider verification.
///
/// # Errors
///
/// Returns a structured error when:
/// - The request is invalid.
/// - The provider identity is invalid.
/// - The provider does not support the request.
/// - Generation fails.
/// - The returned artifact set is invalid.
/// - Provider verification fails.
pub fn generate_documentation<P>(
    provider: &P,
    request: &DocumentationRequest,
) -> Result<DocumentationReport>
where
    P: DocumentationProvider + ?Sized,
{
    request
        .validate()
        .map_err(|error| error.with_operation("doc.request.validate"))?;

    let provider_id =
        DocumentationProviderId::new(provider.id().as_str())
            .map_err(|error| {
                error.with_operation("doc.provider.identity")
            })?;

    let supported = provider
        .supports(request)
        .map_err(|error| {
            error
                .with_operation("doc.provider.compatibility")
                .with_context(format!(
                    "documentation provider: {}",
                    provider_id.as_str()
                ))
        })?;

    if !supported {
        return Err(
            ToolchainError::unsupported_operation(format!(
                "documentation provider '{}' does not support scope '{}' or the requested options",
                provider_id.as_str(),
                request.scope.as_str(),
            ))
            .with_operation("doc.provider.compatibility"),
        );
    }

    let generation = provider
        .generate(request)
        .map_err(|error| {
            error
                .with_operation("doc.generate")
                .with_context(format!(
                    "documentation provider: {}",
                    provider_id.as_str()
                ))
        })?;

    provider
        .verify(request, &generation)
        .map_err(|error| {
            error
                .with_operation("doc.verify")
                .with_context(format!(
                    "documentation provider: {}",
                    provider_id.as_str()
                ))
        })?;

    Ok(DocumentationReport {
        provider: provider_id,
        scope: request.scope.clone(),
        artifacts: generation.into_artifacts(),
        verified: true,
    })
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_identifier(field: &str, value: &str) -> Result<()> {
    if value.is_empty() || value.trim().is_empty() {
        return Err(ToolchainError::invalid_input(format!(
            "{field} must not be empty",
        ))
        .with_operation("doc.request.validate"));
    }

    if value.trim() != value {
        return Err(ToolchainError::invalid_input(format!(
            "{field} must not contain surrounding whitespace",
        ))
        .with_operation("doc.request.validate"));
    }

    if value.chars().any(char::is_control) {
        return Err(ToolchainError::invalid_input(format!(
            "{field} must not contain control characters",
        ))
        .with_operation("doc.request.validate"));
    }

    Ok(())
}

fn validate_path(field: &str, path: &std::path::Path) -> Result<()> {
    if path.as_os_str().is_empty() {
        return Err(ToolchainError::invalid_input(format!(
            "{field} must not be empty",
        ))
        .with_operation("doc.request.validate"));
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicUsize, Ordering};

    use crate::compiler::artifacts::artifact::{
        ArtifactDescriptor,
        ArtifactFormatId,
        ArtifactId,
        ArtifactKindId,
        ArtifactProducerId,
        ArtifactProvenance,
    };

    fn documentation_artifact() -> Artifact {
        let producer = ArtifactProducerId::new("test.provider")
            .expect("valid producer identity");

        let descriptor = ArtifactDescriptor::new(
            ArtifactId::new("test.documentation.1")
                .expect("valid artifact identity"),
            ArtifactKindId::new("documentation")
                .expect("valid artifact kind"),
            ArtifactFormatId::new("text/markdown")
                .expect("valid artifact format"),
            ArtifactProvenance::new(producer),
        );

        Artifact::new_inline(descriptor, b"# Test documentation".to_vec())
    }

    struct TestProvider {
        id: DocumentationProviderId,
        verify_calls: AtomicUsize,
        supported: bool,
    }

    impl TestProvider {
        fn new(supported: bool) -> Self {
            Self {
                id: DocumentationProviderId::new("test.documentation")
                    .expect("valid provider identity"),
                verify_calls: AtomicUsize::new(0),
                supported,
            }
        }
    }

    impl DocumentationProvider for TestProvider {
        fn id(&self) -> &DocumentationProviderId {
            &self.id
        }

        fn supports(&self, _request: &DocumentationRequest) -> Result<bool> {
            Ok(self.supported)
        }

        fn generate(
            &self,
            _request: &DocumentationRequest,
        ) -> Result<DocumentationGeneration> {
            DocumentationGeneration::new(vec![documentation_artifact()])
        }

        fn verify(
            &self,
            _request: &DocumentationRequest,
            generation: &DocumentationGeneration,
        ) -> Result<()> {
            self.verify_calls.fetch_add(1, Ordering::SeqCst);

            if generation.artifacts().is_empty() {
                return Err(ToolchainError::provider_failed(
                    "test provider received empty generation",
                ));
            }

            Ok(())
        }
    }

    #[test]
    fn rejects_empty_provider_identity() {
        assert!(DocumentationProviderId::new(" ").is_err());
    }

    #[test]
    fn rejects_provider_identity_with_surrounding_whitespace() {
        assert!(DocumentationProviderId::new(" provider ").is_err());
    }

    #[test]
    fn accepts_custom_documentation_scope() {
        let scope = DocumentationScope::Custom(
            "organization.internal-api".to_owned(),
        );

        assert!(scope.validate().is_ok());
        assert_eq!(scope.as_str(), "organization.internal-api");
    }

    #[test]
    fn rejects_empty_custom_scope() {
        let scope = DocumentationScope::Custom(String::new());

        assert!(scope.validate().is_err());
    }

    #[test]
    fn accepts_open_ended_domain_identifiers() {
        let options = DocumentationOptions::new()
            .with_domain("quantum")
            .with_domain("future.photonic")
            .with_domain("distributed")
            .with_format("text/markdown")
            .with_format("application/vnd.example.docs");

        assert!(options.validate().is_ok());
        assert_eq!(options.domains.len(), 3);
        assert_eq!(options.formats.len(), 2);
    }

    #[test]
    fn rejects_empty_format_identifiers() {
        let options = DocumentationOptions::new().with_format(" ");

        assert!(options.validate().is_err());
    }

    #[test]
    fn rejects_empty_project_root() {
        let request = DocumentationRequest::new(PathBuf::new());

        assert!(request.validate().is_err());
    }

    #[test]
    fn accepts_valid_request_without_destination() {
        let request = DocumentationRequest::new("project")
            .with_scope(DocumentationScope::All)
            .with_options(
                DocumentationOptions::new()
                    .with_format("text/markdown")
                    .with_domain("classical"),
            );

        assert!(request.validate().is_ok());
    }

    #[test]
    fn rejects_empty_destination() {
        let request = DocumentationRequest::new("project")
            .with_destination(PathBuf::new());

        assert!(request.validate().is_err());
    }

    #[test]
    fn rejects_non_documentation_artifact_kind() {
        let producer = ArtifactProducerId::new("test.provider")
            .expect("valid producer identity");

        let descriptor = ArtifactDescriptor::new(
            ArtifactId::new("test.artifact.1")
                .expect("valid artifact identity"),
            ArtifactKindId::new("native-executable")
                .expect("valid artifact kind"),
            ArtifactFormatId::new("application/octet-stream")
                .expect("valid artifact format"),
            ArtifactProvenance::new(producer),
        );

        let artifact =
            Artifact::new_inline(descriptor, b"not documentation".to_vec());

        assert!(DocumentationGeneration::new(vec![artifact]).is_err());
    }

    #[test]
    fn rejects_empty_generation() {
        assert!(DocumentationGeneration::new(Vec::new()).is_err());
    }

    #[test]
    fn successful_generation_requires_provider_verification() {
        let provider = TestProvider::new(true);

        let request = DocumentationRequest::new("project")
            .with_scope(DocumentationScope::Project)
            .with_options(
                DocumentationOptions::new().with_format("text/markdown"),
            );

        let report = generate_documentation(&provider, &request)
            .expect("documentation generation should succeed");

        assert!(report.is_verified());
        assert_eq!(report.artifact_count(), 1);
        assert_eq!(provider.verify_calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn unsupported_provider_is_not_invoked_for_generation() {
        let provider = TestProvider::new(false);
        let request = DocumentationRequest::new("project");

        assert!(generate_documentation(&provider, &request).is_err());
        assert_eq!(provider.verify_calls.load(Ordering::SeqCst), 0);
    }
}
