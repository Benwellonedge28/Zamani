
#![forbid(unsafe_code)]

//! Shared, immutable context for the Zamani parser.
//!
//! File: `src/frontend/parser/context.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide the immutable services shared by parser components during a
//! parse operation without coupling grammar code to a particular registry,
//! source-map implementation, or configuration representation.
//!
//! # Ownership
//!
//! This module owns:
//! - the shared parser-context abstraction;
//! - the contracts for configuration, grammar registries, and source services;
//! - explicit context construction and access;
//! - cheap context cloning through `Arc`;
//! - documented thread-sharing and lifetime boundaries.
//!
//! This module does not own:
//! - lexer scanning or token representation;
//! - parser cursor positions or mutable parse-session state;
//! - grammar production implementations;
//! - the canonical AST or AST construction;
//! - diagnostic collection or recovery state;
//! - semantic analysis, type checking, or effect checking;
//! - hardware discovery, target selection, allocation, or execution.
//!
//! # Integration
//!
//! `config.rs` supplies the concrete configuration type.
//! `registry.rs` supplies the concrete grammar-extension registry.
//! `source.rs` supplies the concrete source services.
//! `session.rs` owns mutable state for one parse invocation.
//! `parser.rs` coordinates grammar productions.
//! `api.rs` constructs the context and exposes supported entry points.
//!
//! The context deliberately does not import those modules directly. Their
//! implementations can be completed independently and then implement the
//! small contracts defined here. This avoids a circular dependency between
//! context construction and parser orchestration.
//!
//! # Concurrency
//!
//! The context shares services through `Arc`. The service values must be
//! `Send + Sync` to be shared across threads. This does not make mutable
//! parser sessions thread-safe; each concurrent parse must own its own
//! session and cursor.
//!
//! # Scalability
//!
//! This module defines no hardware-capacity limits. It does not impose a
//! fixed count of domains, dialects, operations, devices, qubits, nodes,
//! declarations, or source tokens.
//!
//! `Arc` reference counts and allocations remain subject to the platform's
//! actual representational and resource limits. Resource budgets belong in
//! configuration and are enforced by the consuming parser components.
//!
//! # Determinism
//!
//! The context does not mutate registries, load plugins, or perform
//! environment-dependent discovery. The supplied services must document
//! their own deterministic behavior. A parse is reproducible when its
//! source, configuration, registry snapshot, source services, and parser
//! implementation are equivalent.

use core::fmt;
use std::sync::Arc;

/// Contract for an immutable parser configuration.
///
/// Implement this for the canonical configuration type in `config.rs`.
///
/// The trait intentionally has no hardware-specific fields. Parser options
/// describe syntax, compatibility, recovery, and implementation budgets;
/// target resource negotiation belongs to downstream compiler components.
pub trait ParserConfiguration: Send + Sync + 'static {
    /// Returns the selected language-edition identifier.
    ///
    /// The identifier is opaque to this module. Edition syntax and
    /// compatibility rules are interpreted by the parser and specification.
    fn language_edition(&self) -> &str;
}

/// Contract for an immutable grammar-extension registry snapshot.
///
/// Implement this for the registry defined in `registry.rs`.
///
/// A context receives a registry through shared ownership. It does not
/// register, replace, or activate extensions while parsing. If extensions
/// must change, construct a new registry snapshot and a new context.
///
/// Implementations should resolve duplicate registrations and precedence
/// conflicts before a snapshot is supplied to a parser.
pub trait GrammarRegistry: Send + Sync + 'static {
    /// Returns the stable identity of this registry snapshot.
    ///
    /// The value is intended for tracing, diagnostics, caches, and
    /// reproducibility metadata. It is not a security credential and must
    /// not be used as proof that two registries have identical contents.
    fn snapshot_id(&self) -> &str;
}

/// Contract for immutable source services used by parser components.
///
/// Implement this for the canonical source-map or source-provider adapter
/// in `source.rs`.
///
/// Source services may map source identifiers to source text, names, and
/// location metadata. The parser context does not define a second `Span`
/// type: lexer spans remain the canonical lexical span representation.
pub trait SourceServices: Send + Sync + 'static {
    /// Returns a stable description of the source-service snapshot.
    ///
    /// This identifier is descriptive metadata, not a globally unique ID.
    fn snapshot_id(&self) -> &str;
}

/// Immutable services shared by the parser and its grammar components.
///
/// `C`, `R`, and `S` are the concrete configuration, registry, and source
/// service types. They are generic so that this module does not need to
/// invent placeholder versions of types owned by other modules.
///
/// Values are held behind `Arc`, so cloning a context does not clone the
/// configuration, registry, or source service contents.
///
/// # Type parameters
///
/// - `C`: immutable parser configuration.
/// - `R`: immutable grammar-extension registry snapshot.
/// - `S`: immutable source services.
pub struct ParserContext<C, R, S>
where
    C: ParserConfiguration,
    R: GrammarRegistry,
    S: SourceServices,
{
    config: Arc<C>,
    registry: Arc<R>,
    sources: Arc<S>,
}

impl<C, R, S> ParserContext<C, R, S>
where
    C: ParserConfiguration,
    R: GrammarRegistry,
    S: SourceServices,
{
    /// Creates a context from already-constructed immutable services.
    ///
    /// Construction is intentionally infallible: configuration validation,
    /// extension conflict detection, and source-map validation belong to
    /// the respective owning modules before the context is constructed.
    ///
    /// The supplied `Arc` values are retained without copying their contents.
    #[must_use]
    pub fn new(config: Arc<C>, registry: Arc<R>, sources: Arc<S>) -> Self {
        Self {
            config,
            registry,
            sources,
        }
    }

    /// Creates a context by placing owned services into shared ownership.
    ///
    /// Use this convenience constructor when the caller owns the concrete
    /// values and does not already have `Arc` handles.
    #[must_use]
    pub fn from_owned(config: C, registry: R, sources: S) -> Self {
        Self::new(
            Arc::new(config),
            Arc::new(registry),
            Arc::new(sources),
        )
    }

    /// Borrows the immutable parser configuration.
    #[must_use]
    pub fn config(&self) -> &C {
        self.config.as_ref()
    }

    /// Borrows the immutable grammar-registry snapshot.
    #[must_use]
    pub fn registry(&self) -> &R {
        self.registry.as_ref()
    }

    /// Borrows the immutable source services.
    #[must_use]
    pub fn sources(&self) -> &S {
        self.sources.as_ref()
    }

    /// Returns a shared handle to the configuration.
    ///
    /// This is useful when constructing a downstream component that
    /// explicitly needs to retain configuration beyond the current borrow.
    #[must_use]
    pub fn config_arc(&self) -> Arc<C> {
        Arc::clone(&self.config)
    }

    /// Returns a shared handle to the registry snapshot.
    #[must_use]
    pub fn registry_arc(&self) -> Arc<R> {
        Arc::clone(&self.registry)
    }

    /// Returns a shared handle to the source services.
    #[must_use]
    pub fn sources_arc(&self) -> Arc<S> {
        Arc::clone(&self.sources)
    }

    /// Returns the selected language-edition identifier.
    #[must_use]
    pub fn language_edition(&self) -> &str {
        self.config.language_edition()
    }

    /// Returns the registry snapshot identifier.
    #[must_use]
    pub fn registry_snapshot_id(&self) -> &str {
        self.registry.snapshot_id()
    }

    /// Returns the source-service snapshot identifier.
    #[must_use]
    pub fn source_snapshot_id(&self) -> &str {
        self.sources.snapshot_id()
    }

    /// Creates another context handle sharing the same immutable services.
    ///
    /// This is equivalent to cloning the context, but makes the intended
    /// sharing explicit at call sites.
    #[must_use]
    pub fn shared(&self) -> Self {
        Self::new(
            Arc::clone(&self.config),
            Arc::clone(&self.registry),
            Arc::clone(&self.sources),
        )
    }

    /// Returns the strong-reference count for each shared service.
    ///
    /// This is intended for diagnostics and tests, not synchronization,
    /// correctness decisions, or resource-availability guarantees.
    #[must_use]
    pub fn strong_counts(&self) -> ContextStrongCounts {
        ContextStrongCounts {
            configuration: Arc::strong_count(&self.config),
            registry: Arc::strong_count(&self.registry),
            sources: Arc::strong_count(&self.sources),
        }
    }
}

impl<C, R, S> Clone for ParserContext<C, R, S>
where
    C: ParserConfiguration,
    R: GrammarRegistry,
    S: SourceServices,
{
    fn clone(&self) -> Self {
        self.shared()
    }
}

impl<C, R, S> fmt::Debug for ParserContext<C, R, S>
where
    C: ParserConfiguration,
    R: GrammarRegistry,
    S: SourceServices,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ParserContext")
            .field("language_edition", &self.language_edition())
            .field("registry_snapshot_id", &self.registry_snapshot_id())
            .field("source_snapshot_id", &self.source_snapshot_id())
            .field("strong_counts", &self.strong_counts())
            .finish_non_exhaustive()
    }
}

/// Diagnostic snapshot of the context's shared-reference counts.
///
/// These counts can change as other context handles are cloned or dropped.
/// They are informational only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContextStrongCounts {
    /// Strong references to the configuration.
    pub configuration: usize,

    /// Strong references to the registry snapshot.
    pub registry: usize,

    /// Strong references to the source services.
    pub sources: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[derive(Debug)]
    struct TestConfig {
        edition: String,
    }

    impl ParserConfiguration for TestConfig {
        fn language_edition(&self) -> &str {
            &self.edition
        }
    }

    #[derive(Debug)]
    struct TestRegistry {
        id: String,
    }

    impl GrammarRegistry for TestRegistry {
        fn snapshot_id(&self) -> &str {
            &self.id
        }
    }

    #[derive(Debug)]
    struct TestSources {
        id: String,
    }

    impl SourceServices for TestSources {
        fn snapshot_id(&self) -> &str {
            &self.id
        }
    }

    fn context() -> ParserContext<TestConfig, TestRegistry, TestSources> {
        ParserContext::from_owned(
            TestConfig {
                edition: "test-edition".to_owned(),
            },
            TestRegistry {
                id: "registry-test".to_owned(),
            },
            TestSources {
                id: "sources-test".to_owned(),
            },
        )
    }

    #[test]
    fn exposes_the_supplied_services() {
        let context = context();

        assert_eq!(context.language_edition(), "test-edition");
        assert_eq!(context.registry_snapshot_id(), "registry-test");
        assert_eq!(context.source_snapshot_id(), "sources-test");
    }

    #[test]
    fn clones_share_the_same_service_allocations() {
        let context = context();
        let before = context.strong_counts();

        let cloned = context.clone();
        let after = context.strong_counts();

        assert_eq!(before.configuration + 1, after.configuration);
        assert_eq!(before.registry + 1, after.registry);
        assert_eq!(before.sources + 1, after.sources);

        assert_eq!(cloned.language_edition(), context.language_edition());
        assert_eq!(
            cloned.registry_snapshot_id(),
            context.registry_snapshot_id()
        );
    }

    #[test]
    fn explicit_shared_handle_preserves_snapshot_identity() {
        let context = context();
        let shared = context.shared();

        assert_eq!(
            context.registry_snapshot_id(),
            shared.registry_snapshot_id()
        );
        assert_eq!(
            context.source_snapshot_id(),
            shared.source_snapshot_id()
        );
    }

    #[test]
    fn accessors_return_the_original_shared_allocations() {
        let context = context();

        let config = context.config_arc();
        let registry = context.registry_arc();
        let sources = context.sources_arc();

        assert!(Arc::ptr_eq(&config, &context.config));
        assert!(Arc::ptr_eq(&registry, &context.registry));
        assert!(Arc::ptr_eq(&sources, &context.sources));
    }

    #[test]
    fn debug_output_reports_metadata_without_dumping_source_contents() {
        let context = context();
        let rendered = format!("{context:?}");

        assert!(rendered.contains("test-edition"));
        assert!(rendered.contains("registry-test"));
        assert!(rendered.contains("sources-test"));
    }
}
