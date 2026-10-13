
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Analysis Context
//!
//! File: `src/frontend/semantic/context.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide the immutable, shareable context used by one or more semantic
//! analysis sessions. The context supplies validated configuration and a
//! stable snapshot of the registered semantic extensions available to the
//! analysis implementation.
//!
//! ## STATUS
//!
//! Implementation of the semantic-context boundary. Full integration depends
//! on the semantic module exports and extension registry implementing the
//! contracts documented here.
//!
//! ## AUTHORITY
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/extensibility.md`
//! - `grammar/Zamani.g4`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/config.rs`
//!
//! Normative language specifications take precedence over implementation
//! convenience.
//!
//! ## OWNS
//!
//! - The immutable semantic configuration used by an analysis.
//! - The immutable extension-catalog snapshot visible to an analysis.
//! - The contract for registry identity and revision.
//! - Construction-time validation of the context boundary.
//! - Read-only access to these shared services.
//!
//! ## DOES_NOT_OWN
//!
//! - Source text, tokens, AST nodes, source spans, or source maps.
//! - Symbol tables, declaration resolution, or scope mutation.
//! - Type inference, unification, or type compatibility.
//! - Effect inference or effect authorization decisions.
//! - Capability evidence or target capability discovery.
//! - Resource allocation, target feasibility, placement, routing,
//!   scheduling, or execution.
//! - Contract evaluation or domain-specific semantic rules.
//! - Mutable per-analysis state, diagnostics, inference variables,
//!   work accounting, recovery, or checkpoints.
//! - Classical IR, `quantum::ir`, HDL IR, lowering, or runtime state.
//!
//! ## ARCHITECTURAL POSITION
//!
//! ```text
//! Canonical AST + semantic configuration
//!                  |
//!                  v
//!          SemanticContext
//!          /             \
//!   configuration    registry snapshot
//!          \             /
//!           v           v
//!          SemanticSession
//!                  |
//!                  v
//!          Semantic pipeline
//!                  |
//!                  v
//!         Validated semantic model
//!                  |
//!                  v
//!       Canonical domain IR boundaries
//! ```
//!
//! The context describes the services available to analysis. It does not
//! perform analysis and does not certify that a program is semantically valid.
//!
//! ## IMMUTABILITY AND CONCURRENCY
//!
//! A context owns no mutable global state. It can be shared by independent
//! sessions through `Arc<SemanticContext>`.
//!
//! The registry supplied to the context must represent an immutable snapshot.
//! Implementations must not mutate the meaning of an existing snapshot after
//! it has been published. Registry updates create a new snapshot with a new
//! revision.
//!
//! A registry implementation must be `Send + Sync` because a context may be
//! shared across threads. This does not require the analyzer to execute its
//! passes concurrently.
//!
//! ## DETERMINISM
//!
//! For reproducible analysis, the same source, configuration, implementation
//! version, and registry snapshot must produce equivalent semantic results.
//!
//! A registry identity consists of a stable namespace and a revision supplied
//! by its owner. The owner must ensure that a revision uniquely identifies
//! the contents of a snapshot within that namespace and its documented
//! compatibility scope.
//!
//! This module does not use timestamps, random values, memory addresses,
//! environment variables, filesystem state, network access, or hardware
//! discovery to establish context identity.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This context introduces no language-level maximum for declarations,
//! types, dimensions, qubits, processors, devices, nodes, memory, tensor rank,
//! hardware width, or computational domains.
//!
//! Operational budgets are provided by `SemanticConfig`. The context does
//! not enforce them and does not reinterpret them as hardware limits.
//!
//! No finite compiler can guarantee processing an actually infinite input.
//! This contract avoids artificial semantic ceilings while allowing the
//! analysis session and operating environment to enforce explicit,
//! reportable operational limits.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `config.rs` owns `SemanticConfig`, language-version policy, extension
//!   policy, diagnostic policy, and operational work budgets.
//! - `api.rs` constructs a context after validating requested language and
//!   implementation compatibility.
//! - `domains/registry.rs` implements `SemanticExtensionCatalog` using an
//!   immutable registry snapshot.
//! - `session.rs` borrows or shares this context and owns mutable state for
//!   one analysis invocation.
//! - `pipeline.rs` uses the same context for every pass in an analysis.
//! - `incremental/` includes configuration and registry identity/revision
//!   in cache identity whenever they affect computed results.
//! - `symbols/`, `types/`, `effects/`, `capabilities/`, `resources/`,
//!   `contracts/`, and `domains/` consume context services without creating
//!   competing global configuration or registries.
//! - Domain validators must not perform target discovery through this context.
//! - Downstream IR and target planners consume validated semantic facts;
//!   they do not redefine the context's language version or registry snapshot.
//!
//! ## FREEZE CRITERIA
//!
//! This file is ready to freeze when:
//!
//! 1. `config.rs` exposes the documented configuration API.
//! 2. The production extension registry implements this file's catalog trait.
//! 3. `mod.rs` declares and exports the context API.
//! 4. `api.rs`, `session.rs`, and `pipeline.rs` use this shared contract.
//! 5. Context construction and invalid-configuration tests pass.
//! 6. Registry identity and revision behavior are tested.
//! 7. Concurrent read-only use is tested with the production registry.
//! 8. Workspace tests and lint checks pass on the supported Rust toolchain.

use core::fmt;
use std::sync::Arc;

use super::config::{ExtensionPolicy, SemanticConfig};

/// Stable identity for one immutable semantic-extension registry snapshot.
///
/// The namespace identifies the registry owner or registry family. The
/// revision identifies the exact immutable snapshot within that namespace.
///
/// The registry owner is responsible for ensuring that two different
/// snapshots in the same namespace do not share a revision when their
/// semantic contents differ.
///
/// A revision is not a hardware version, language version, source-file ID,
/// timestamp, or memory address.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegistryIdentity {
    namespace: Arc<str>,
    revision: u64,
}

impl RegistryIdentity {
    /// Construct a registry identity.
    ///
    /// # Errors
    ///
    /// Returns `RegistryIdentityError::EmptyNamespace` when the namespace
    /// is empty or consists only of whitespace.
    pub fn new(
        namespace: impl Into<Arc<str>>,
        revision: u64,
    ) -> Result<Self, RegistryIdentityError> {
        let namespace = namespace.into();

        if namespace.trim().is_empty() {
            return Err(RegistryIdentityError::EmptyNamespace);
        }

        Ok(Self {
            namespace,
            revision,
        })
    }

    /// Return the registry namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the immutable snapshot revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }
}

/// Errors encountered while constructing a registry identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RegistryIdentityError {
    /// A registry namespace cannot be empty or whitespace-only.
    EmptyNamespace,
}

impl fmt::Display for RegistryIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNamespace => {
                f.write_str("semantic registry namespace must not be empty")
            }
        }
    }
}

impl std::error::Error for RegistryIdentityError {}

/// Read-only interface to an immutable semantic-extension registry snapshot.
///
/// The registry owns extension definitions, compatibility checks, and
/// registration rules. The context owns only a shared reference to the
/// published snapshot.
///
/// Extension identifiers are qualified, stable identifiers defined by the
/// registry contract. This trait deliberately does not enumerate a permanent
/// set of computational domains or built-in operations.
///
/// Implementations must:
///
/// - return the same identity and revision for the lifetime of a snapshot;
/// - return deterministic answers for unchanged snapshot contents;
/// - avoid interior mutation that changes semantic meaning after publication;
/// - validate extension-policy compatibility without performing target
///   discovery or executing extension code;
/// - document the namespace and revision compatibility guarantees.
pub trait SemanticExtensionCatalog: fmt::Debug + Send + Sync {
    /// Return the stable identity of this immutable snapshot.
    fn identity(&self) -> &RegistryIdentity;

    /// Return the number of registered extension definitions in this snapshot.
    ///
    /// This is catalog metadata, not a limit on how many extensions Zamani
    /// may support. The implementation must not use this count to truncate
    /// registry contents.
    fn extension_count(&self) -> usize;

    /// Determine whether the snapshot contains the given qualified ID.
    ///
    /// This query must not execute the extension or mutate the registry.
    fn contains(&self, qualified_id: &str) -> bool;

    /// Validate this snapshot against the configured extension policy.
    ///
    /// Implementations may reject incompatible or internally inconsistent
    /// registrations. They must not reject a valid source program merely
    /// because a particular target lacks a capability.
    ///
    /// The default implementation accepts the snapshot. Production
    /// registries should override this method when they enforce extension
    /// stability, compatibility, or activation rules.
    fn validate_policy(&self, _policy: ExtensionPolicy) -> Result<(), String> {
        Ok(())
    }
}

/// Empty immutable catalog used when semantic extensions are not supplied.
///
/// It is useful for minimal programs, tests, and early integration stages.
/// It does not mean that Zamani itself is restricted to a fixed set of
/// domains. Callers that enable registered extensions must supply the
/// corresponding production catalog.
#[derive(Debug, Clone)]
pub struct EmptySemanticExtensionCatalog {
    identity: RegistryIdentity,
}

impl EmptySemanticExtensionCatalog {
    /// Create an empty catalog with a stable, explicit identity.
    #[must_use]
    pub fn new() -> Self {
        Self {
            identity: RegistryIdentity {
                namespace: Arc::<str>::from("zamani.semantic.empty"),
                revision: 0,
            },
        }
    }
}

impl Default for EmptySemanticExtensionCatalog {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticExtensionCatalog for EmptySemanticExtensionCatalog {
    fn identity(&self) -> &RegistryIdentity {
        &self.identity
    }

    fn extension_count(&self) -> usize {
        0
    }

    fn contains(&self, _qualified_id: &str) -> bool {
        false
    }
}

/// Immutable services shared by semantic analysis passes.
///
/// `SemanticContext` is intentionally independent of the AST's concrete
/// root type. It can be shared by analysis sessions that operate on the
/// canonical AST without defining another AST or coupling semantic analysis
/// to a parser implementation.
///
/// Construct the context through [`SemanticContext::new`] or
/// [`SemanticContext::without_extensions`] so configuration and registry
/// contracts are validated before analysis begins.
#[derive(Clone)]
pub struct SemanticContext {
    config: Arc<SemanticConfig>,
    extensions: Arc<dyn SemanticExtensionCatalog>,
}

impl SemanticContext {
    /// Construct a context from validated configuration and a registry
    /// snapshot.
    ///
    /// The configuration and catalog are retained as immutable shared
    /// services. Callers may share the resulting context across independent
    /// sessions.
    ///
    /// # Errors
    ///
    /// Returns an error when configuration validation or the registry's
    /// extension-policy validation fails.
    pub fn new(
        config: SemanticConfig,
        extensions: Arc<dyn SemanticExtensionCatalog>,
    ) -> Result<Self, SemanticContextError> {
        config
            .validate()
            .map_err(|error| SemanticContextError::InvalidConfiguration {
                message: error.to_string(),
            })?;

        if extensions.identity().namespace().trim().is_empty() {
            return Err(SemanticContextError::InvalidRegistryIdentity);
        }

        extensions
            .validate_policy(config.extension_policy())
            .map_err(|message| SemanticContextError::ExtensionPolicy {
                message,
            })?;

        Ok(Self {
            config: Arc::new(config),
            extensions,
        })
    }

    /// Construct a context using the empty extension catalog.
    ///
    /// This constructor is appropriate when the analysis does not require
    /// extension definitions. If a language construct requires an extension
    /// that is not registered, the relevant semantic pass must emit a
    /// structured diagnostic; it must not silently accept the construct.
    ///
    /// # Errors
    ///
    /// Returns an error if configuration validation fails.
    pub fn without_extensions(
        config: SemanticConfig,
    ) -> Result<Self, SemanticContextError> {
        Self::new(
            config,
            Arc::new(EmptySemanticExtensionCatalog::new()),
        )
    }

    /// Return the immutable semantic configuration.
    #[must_use]
    pub fn config(&self) -> &SemanticConfig {
        self.config.as_ref()
    }

    /// Return the immutable extension-catalog snapshot.
    #[must_use]
    pub fn extensions(&self) -> &dyn SemanticExtensionCatalog {
        self.extensions.as_ref()
    }

    /// Return the stable identity of the extension snapshot.
    ///
    /// Incremental analysis must include this identity in its cache key
    /// whenever registered extensions can affect a result.
    #[must_use]
    pub fn registry_identity(&self) -> &RegistryIdentity {
        self.extensions.identity()
    }

    /// Return a clone of the shared configuration handle.
    ///
    /// This is useful when a session needs to retain the configuration
    /// independently of the context's borrow lifetime.
    #[must_use]
    pub fn shared_config(&self) -> Arc<SemanticConfig> {
        Arc::clone(&self.config)
    }

    /// Return a clone of the shared extension-catalog handle.
    ///
    /// Cloning the handle does not clone or mutate the registry snapshot.
    #[must_use]
    pub fn shared_extensions(&self) -> Arc<dyn SemanticExtensionCatalog> {
        Arc::clone(&self.extensions)
    }
}

impl fmt::Debug for SemanticContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SemanticContext")
            .field("config", self.config.as_ref())
            .field("registry_identity", self.extensions.identity())
            .field("extension_count", &self.extensions.extension_count())
            .finish()
    }
}

/// Errors raised while constructing a semantic context.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SemanticContextError {
    /// The semantic configuration did not satisfy its invariants.
    InvalidConfiguration {
        /// Human-readable explanation supplied by the configuration validator.
        message: String,
    },

    /// The extension registry rejected the configured extension policy.
    ExtensionPolicy {
        /// Human-readable explanation supplied by the registry.
        message: String,
    },

    /// The registry exposed an invalid identity.
    ///
    /// `RegistryIdentity::new` prevents this for identities created through
    /// the public constructor. This variant also protects the context
    /// boundary against malformed custom implementations.
    InvalidRegistryIdentity,
}

impl fmt::Display for SemanticContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { message } => {
                write!(f, "invalid semantic configuration: {message}")
            }
            Self::ExtensionPolicy { message } => {
                write!(f, "semantic extension policy rejected: {message}")
            }
            Self::InvalidRegistryIdentity => {
                f.write_str("semantic extension registry has an invalid identity")
            }
        }
    }
}

impl std::error::Error for SemanticContextError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::config::{
        AnalysisMode, LanguageVersion,
    };

    #[derive(Debug)]
    struct TestCatalog {
        identity: RegistryIdentity,
        ids: Vec<String>,
    }

    impl TestCatalog {
        fn new(
            namespace: &str,
            revision: u64,
            ids: &[&str],
        ) -> Self {
            Self {
                identity: RegistryIdentity::new(namespace, revision)
                    .expect("test namespace is valid"),
                ids: ids.iter().map(|id| (*id).to_owned()).collect(),
            }
        }
    }

    impl SemanticExtensionCatalog for TestCatalog {
        fn identity(&self) -> &RegistryIdentity {
            &self.identity
        }

        fn extension_count(&self) -> usize {
            self.ids.len()
        }

        fn contains(&self, qualified_id: &str) -> bool {
            self.ids.iter().any(|id| id == qualified_id)
        }
    }

    #[test]
    fn registry_identity_rejects_empty_namespaces() {
        assert_eq!(
            RegistryIdentity::new("", 0),
            Err(RegistryIdentityError::EmptyNamespace)
        );
        assert_eq!(
            RegistryIdentity::new("   ", 0),
            Err(RegistryIdentityError::EmptyNamespace)
        );
    }

    #[test]
    fn registry_identity_preserves_namespace_and_revision() {
        let identity = RegistryIdentity::new("example.domain", 42)
            .expect("namespace is valid");

        assert_eq!(identity.namespace(), "example.domain");
        assert_eq!(identity.revision(), 42);
    }

    #[test]
    fn empty_catalog_has_no_registered_extensions() {
        let catalog = EmptySemanticExtensionCatalog::new();

        assert_eq!(catalog.extension_count(), 0);
        assert!(!catalog.contains("example.domain.operation"));
        assert_eq!(
            catalog.identity().namespace(),
            "zamani.semantic.empty"
        );
    }

    #[test]
    fn context_constructs_with_default_configuration() {
        let context = SemanticContext::without_extensions(
            SemanticConfig::default(),
        )
        .expect("default configuration is valid");

        assert_eq!(
            context.config().language_version(),
            LanguageVersion::new(1, 0, 0)
        );
        assert_eq!(context.extensions().extension_count(), 0);
        assert_eq!(
            context.registry_identity().namespace(),
            "zamani.semantic.empty"
        );
    }

    #[test]
    fn context_exposes_a_registered_snapshot_without_mutating_it() {
        let catalog = Arc::new(TestCatalog::new(
            "example.semantic",
            7,
            &[
                "example.quantum.operation",
                "example.tensor.operation",
            ],
        ));

        let context = SemanticContext::new(
            SemanticConfig::default(),
            catalog.clone(),
        )
        .expect("test catalog is valid");

        assert_eq!(context.extensions().extension_count(), 2);
        assert!(context
            .extensions()
            .contains("example.quantum.operation"));
        assert!(context
            .extensions()
            .contains("example.tensor.operation"));
        assert!(!context
            .extensions()
            .contains("example.unknown.operation"));

        assert_eq!(
            context.registry_identity().namespace(),
            "example.semantic"
        );
        assert_eq!(context.registry_identity().revision(), 7);
        assert_eq!(catalog.extension_count(), 2);
    }

    #[test]
    fn context_clones_share_the_same_immutable_services() {
        let context = SemanticContext::without_extensions(
            SemanticConfig::default(),
        )
        .expect("default configuration is valid");

        let cloned = context.clone();

        assert!(Arc::ptr_eq(
            &context.shared_config(),
            &cloned.shared_config()
        ));
        assert!(Arc::ptr_eq(
            &context.shared_extensions(),
            &cloned.shared_extensions()
        ));
    }

    #[test]
    fn context_debug_output_is_safe_and_useful() {
        let context = SemanticContext::without_extensions(
            SemanticConfig::default()
                .with_analysis_mode(AnalysisMode::Strict),
        )
        .expect("default configuration is valid");

        let debug = format!("{context:?}");

        assert!(debug.contains("SemanticContext"));
        assert!(debug.contains("registry_identity"));
        assert!(debug.contains("extension_count"));
    }

    #[test]
    fn custom_catalog_identity_changes_with_revision() {
        let first = TestCatalog::new("example.semantic", 1, &[]);
        let second = TestCatalog::new("example.semantic", 2, &[]);

        assert_ne!(first.identity(), second.identity());
    }
}
