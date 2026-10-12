
#![forbid(unsafe_code)]

//! Domain-scoped grammar extension catalog for Zamani.
//!
//! File: src/frontend/parser/domains/extensions.rs
//! Minimum Rust version: 1.97
//! Edition: Rust 2021
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide a domain-level composition boundary for classical, quantum, HDL,
//! hybrid, distributed, data-oriented, and future grammar extensions.
//!
//! This module reuses the canonical extension contracts from
//! `crate::frontend::parser::extensions`. It does not define another extension
//! trait, identifier type, version type, parser, token model, or AST.
//!
//! # Ownership
//!
//! This module owns:
//! - domain-scoped collection of grammar extension handlers;
//! - deterministic catalog construction;
//! - duplicate extension detection;
//! - parser API compatibility validation;
//! - declared-conflict validation;
//! - stable extension iteration after successful construction;
//! - domain-catalog construction errors.
//!
//! This module does not own:
//! - tokenization or token metadata;
//! - parser cursor state or source management;
//! - grammar productions or extension dispatch execution;
//! - canonical AST construction;
//! - semantic analysis, type checking, or effect checking;
//! - capability resolution or resource feasibility;
//! - hardware discovery, allocation, scheduling, or lowering;
//! - dynamic library loading or execution of extension code.
//!
//! # Integration contract
//!
//! - `super::super::extensions` owns `GrammarExtension`, `ExtensionId`,
//!   `ExtensionDescriptor`, `ExtensionVersion`, and extension errors.
//! - `super::super::grammar` owns dispatch and parse-progress enforcement.
//! - `super::super::registry` must eventually use the same canonical
//!   extension identity and descriptor types; it must not maintain a
//!   competing extension model.
//! - `context.rs` and `session.rs` own per-parse mutable state and budgets.
//! - `ast_adapter.rs` owns conversion into the repository's canonical AST.
//! - domain grammar modules own the syntax they recognize.
//!
//! The exact canonical syntax-node type `N` is selected by the caller.
//! A catalog is immutable after construction and can be shared between parse
//! sessions when its extension implementations are safe to share.
//!
//! # Determinism
//!
//! Catalog iteration is ordered by descending extension priority, then by
//! canonical extension ID. Registration order does not decide precedence.
//! Equal-priority matches are still handled by the canonical dispatcher;
//! ordering alone never resolves a grammar ambiguity.
//!
//! # Scalability
//!
//! There is no hard-coded limit on the number of extensions, domains,
//! operations, devices, qubits, registers, nodes, or other target resources.
//! Actual allocation is bounded by available memory and any budgets imposed
//! by the caller. This module does not impose parser-work budgets itself.
//!
//! # Security
//!
//! Registering an extension does not authorize it to bypass parser budgets,
//! access hardware, execute source code, or mutate global parser state.
//! Extension handlers must enforce the canonical `GrammarExtension` contract.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use super::super::extensions::{
    ExtensionDescriptor,
    ExtensionError,
    ExtensionErrorKind,
    ExtensionId,
    ExtensionVersion,
    GrammarExtension,
};

// -----------------------------------------------------------------------------
// Domain identity
// -----------------------------------------------------------------------------

/// Stable identifier for the domain that owns or composes a catalog.
///
/// The value uses the same canonical identifier rules as `ExtensionId`.
/// It identifies a grammar domain, not a physical target or vendor device.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DomainId(ExtensionId);

impl DomainId {
    /// Create a domain identifier such as `core.quantum` or `core.hdl`.
    pub fn new(value: impl Into<String>) -> Result<Self, DomainExtensionError> {
        ExtensionId::new(value)
            .map(Self)
            .map_err(DomainExtensionError::ExtensionContract)
    }

    /// Return the canonical domain identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for DomainId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// -----------------------------------------------------------------------------
// Catalog errors
// -----------------------------------------------------------------------------

/// Errors encountered while building or validating a domain catalog.
///
/// Extension contract errors retain their canonical structured error type.
/// Catalog-specific errors are deterministic and machine-readable.
#[derive(Debug, Clone)]
pub enum DomainExtensionError {
    /// The canonical extension contract rejected metadata.
    ExtensionContract(ExtensionError),

    /// An extension identifier is already present in this catalog.
    DuplicateExtension {
        domain: DomainId,
        extension: ExtensionId,
    },

    /// Two registered extensions explicitly declare a conflict.
    DeclaredConflict {
        domain: DomainId,
        first: ExtensionId,
        second: ExtensionId,
    },

    /// An extension does not support the parser API requested by the caller.
    IncompatibleParserApi {
        domain: DomainId,
        extension: ExtensionId,
        major: u64,
        minor: u64,
    },

    /// The catalog has already been consumed by `build`.
    BuilderConsumed {
        domain: DomainId,
    },
}

impl fmt::Display for DomainExtensionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExtensionContract(error) => write!(formatter, "{error}"),

            Self::DuplicateExtension { domain, extension } => write!(
                formatter,
                "domain {domain} registers extension {extension} more than once"
            ),

            Self::DeclaredConflict {
                domain,
                first,
                second,
            } => write!(
                formatter,
                "domain {domain} contains conflicting extensions \
                 {first} and {second}"
            ),

            Self::IncompatibleParserApi {
                domain,
                extension,
                major,
                minor,
            } => write!(
                formatter,
                "extension {extension} in domain {domain} does not support \
                 parser API {major}.{minor}"
            ),

            Self::BuilderConsumed { domain } => write!(
                formatter,
                "domain extension builder for {domain} has already been consumed"
            ),
        }
    }
}

impl Error for DomainExtensionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ExtensionContract(error) => Some(error),
            _ => None,
        }
    }
}

// -----------------------------------------------------------------------------
// Catalog entry
// -----------------------------------------------------------------------------

/// Immutable extension metadata paired with its shared handler.
///
/// The descriptor is captured when the handler is registered. Catalog
/// validation uses that snapshot, and the handler contract requires its
/// `descriptor()` to remain stable for its lifetime.
#[derive(Clone)]
pub struct DomainExtension<N> {
    descriptor: ExtensionDescriptor,
    handler: Arc<dyn GrammarExtension<N>>,
}

impl<N> DomainExtension<N> {
    fn new(handler: Arc<dyn GrammarExtension<N>>) -> Self {
        Self {
            descriptor: handler.descriptor().clone(),
            handler,
        }
    }

    /// Return the validated descriptor snapshot.
    #[must_use]
    pub fn descriptor(&self) -> &ExtensionDescriptor {
        &self.descriptor
    }

    /// Return the registered handler.
    #[must_use]
    pub fn handler(&self) -> &Arc<dyn GrammarExtension<N>> {
        &self.handler
    }

    /// Return the extension's canonical identifier.
    #[must_use]
    pub fn id(&self) -> &ExtensionId {
        self.descriptor.id()
    }

    /// Return the extension's declared version.
    #[must_use]
    pub const fn version(&self) -> ExtensionVersion {
        self.descriptor.version()
    }
}

// -----------------------------------------------------------------------------
// Mutable builder
// -----------------------------------------------------------------------------

/// Builder for a single domain's extension catalog.
///
/// Build catalogs during frontend initialization, validate them once, then
/// share the resulting immutable catalog between parse sessions.
///
/// A builder is intentionally not internally synchronized: initialization
/// should have one clear owner. The completed catalog can be shared through
/// `Arc` or another repository-approved ownership mechanism.
pub struct DomainExtensionBuilder<N> {
    domain: DomainId,
    extensions: BTreeMap<ExtensionId, DomainExtension<N>>,
}

impl<N> DomainExtensionBuilder<N> {
    /// Create an empty builder for a domain.
    pub fn new(domain: DomainId) -> Self {
        Self {
            domain,
            extensions: BTreeMap::new(),
        }
    }

    /// Return the domain represented by this builder.
    #[must_use]
    pub fn domain(&self) -> &DomainId {
        &self.domain
    }

    /// Number of extensions registered so far.
    #[must_use]
    pub fn len(&self) -> usize {
        self.extensions.len()
    }

    /// Whether no extensions have been registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.extensions.is_empty()
    }

    /// Register a shared extension handler.
    ///
    /// Duplicate IDs are rejected. The handler must provide a stable
    /// descriptor and obey the canonical extension contract.
    ///
    /// This operation does not execute the handler or probe source text.
    pub fn register(
        &mut self,
        handler: Arc<dyn GrammarExtension<N>>,
    ) -> Result<(), DomainExtensionError> {
        let entry = DomainExtension::new(handler);
        let id = entry.id().clone();

        if self.extensions.contains_key(&id) {
            return Err(DomainExtensionError::DuplicateExtension {
                domain: self.domain.clone(),
                extension: id,
            });
        }

        self.extensions.insert(id, entry);
        Ok(())
    }

    /// Register an extension by value when it is `'static`.
    pub fn register_owned<E>(
        &mut self,
        handler: E,
    ) -> Result<(), DomainExtensionError>
    where
        E: GrammarExtension<N> + 'static,
    {
        self.register(Arc::new(handler))
    }

    /// Validate and freeze the catalog for a parser API version.
    ///
    /// Successful construction guarantees:
    ///
    /// - unique extension IDs;
    /// - compatibility with the requested parser API version;
    /// - no explicitly declared conflict between registered extensions;
    /// - stable iteration independent of registration order.
    ///
    /// This method does not claim that all grammar matches are unambiguous.
    /// Match ambiguity depends on actual input and is checked by the
    /// canonical grammar dispatcher.
    pub fn build(
        self,
        parser_api_major: u64,
        parser_api_minor: u64,
    ) -> Result<DomainExtensionCatalog<N>, DomainExtensionError> {
        let ids: BTreeSet<ExtensionId> = self.extensions.keys().cloned().collect();

        // Validate compatibility in stable ID order.
        for (id, entry) in &self.extensions {
            if !entry
                .descriptor()
                .supports_parser_api(parser_api_major, parser_api_minor)
            {
                return Err(DomainExtensionError::IncompatibleParserApi {
                    domain: self.domain.clone(),
                    extension: id.clone(),
                    major: parser_api_major,
                    minor: parser_api_minor,
                });
            }
        }

        // Conflicts are symmetric: a declaration by either side is sufficient.
        // Inspect each unordered pair once in deterministic ID order.
        for (id, entry) in &self.extensions {
            for conflict in entry.descriptor().conflicts() {
                if ids.contains(conflict) && id < conflict {
                    return Err(DomainExtensionError::DeclaredConflict {
                        domain: self.domain.clone(),
                        first: id.clone(),
                        second: conflict.clone(),
                    });
                }

                if ids.contains(conflict) && conflict < id {
                    let other = self
                        .extensions
                        .get(conflict)
                        .expect("registered ID set and catalog keys must agree");

                    if other.descriptor().conflicts_with(id) {
                        return Err(DomainExtensionError::DeclaredConflict {
                            domain: self.domain.clone(),
                            first: conflict.clone(),
                            second: id.clone(),
                        });
                    }

                    // The pair was already checked from the smaller ID.
                    // A one-sided declaration is still a conflict.
                    return Err(DomainExtensionError::DeclaredConflict {
                        domain: self.domain.clone(),
                        first: conflict.clone(),
                        second: id.clone(),
                    });
                }
            }
        }

        // Deterministic precedence: descending priority, then ascending ID.
        // The catalog's map already gives stable ID iteration; sorting here
        // makes the dispatch order explicit and independent of insertion order.
        let mut extensions: Vec<_> = self.extensions.into_values().collect();

        extensions.sort_by(|left, right| {
            right
                .descriptor()
                .priority()
                .cmp(&left.descriptor().priority())
                .then_with(|| left.id().cmp(right.id()))
        });

        Ok(DomainExtensionCatalog {
            domain: self.domain,
            extensions,
            parser_api_major,
            parser_api_minor,
        })
    }
}

// -----------------------------------------------------------------------------
// Immutable catalog
// -----------------------------------------------------------------------------

/// Validated, immutable collection of extensions for one domain.
///
/// The catalog contains no mutable parser-session state. It may be reused
/// across independent parses as long as extension implementations honor the
/// `Send + Sync` contract.
pub struct DomainExtensionCatalog<N> {
    domain: DomainId,
    extensions: Vec<DomainExtension<N>>,
    parser_api_major: u64,
    parser_api_minor: u64,
}

impl<N> DomainExtensionCatalog<N> {
    /// Domain identifier.
    #[must_use]
    pub fn domain(&self) -> &DomainId {
        &self.domain
    }

    /// Parser API major version used during validation.
    #[must_use]
    pub const fn parser_api_major(&self) -> u64 {
        self.parser_api_major
    }

    /// Parser API minor version used during validation.
    #[must_use]
    pub const fn parser_api_minor(&self) -> u64 {
        self.parser_api_minor
    }

    /// Number of registered extensions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.extensions.len()
    }

    /// Whether this catalog contains no extensions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.extensions.is_empty()
    }

    /// Iterate in deterministic dispatch-priority order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &DomainExtension<N>> {
        self.extensions.iter()
    }

    /// Look up an extension by canonical identifier.
    #[must_use]
    pub fn get(&self, id: &ExtensionId) -> Option<&DomainExtension<N>> {
        self.extensions.iter().find(|entry| entry.id() == id)
    }

    /// Borrow handlers in the catalog's deterministic order.
    ///
    /// This adapter allows integration with the canonical dispatcher without
    /// introducing another parser API or another extension trait.
    pub fn handlers(&self) -> Vec<Arc<dyn GrammarExtension<N>>> {
        self.extensions
            .iter()
            .map(|entry| Arc::clone(entry.handler()))
            .collect()
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::parser::extensions::{
        ExtensionApiCompatibility,
        ExtensionDescriptor,
        ExtensionPoint,
        ExtensionVersion,
    };

    struct TestExtension {
        descriptor: ExtensionDescriptor,
    }

    impl TestExtension {
        fn new(id: &str, priority: i32) -> Self {
            let id = ExtensionId::new(id).expect("valid test extension ID");
            let compatibility =
                ExtensionApiCompatibility::new(1, 0, None).expect("valid API range");

            let descriptor = ExtensionDescriptor::new(
                id,
                ExtensionVersion::new(1, 0, 0),
                compatibility,
                [ExtensionPoint::Item],
            )
            .expect("valid extension descriptor")
            .with_priority(priority);

            Self { descriptor }
        }
    }

    impl GrammarExtension<()> for TestExtension {
        fn descriptor(&self) -> &ExtensionDescriptor {
            &self.descriptor
        }

        fn probe(
            &self,
            _point: ExtensionPoint,
            _lookahead: &[crate::frontend::lexer::token_kind::TokenKind],
        ) -> crate::frontend::parser::extensions::ExtensionMatch {
            crate::frontend::parser::extensions::ExtensionMatch::NoMatch
        }

        fn parse(
            &self,
            _point: ExtensionPoint,
            _context: &mut dyn crate::frontend::parser::extensions::ExtensionParseContext,
        ) -> Result<(), ExtensionError> {
            Err(ExtensionError::new(
                ExtensionErrorKind::MalformedConstruct,
                "test extension should not be invoked",
            ))
        }
    }

    fn domain() -> DomainId {
        DomainId::new("test.domain").expect("valid domain ID")
    }

    #[test]
    fn rejects_invalid_domain_identifier() {
        assert!(DomainId::new("").is_err());
        assert!(DomainId::new(" test.domain").is_err());
        assert!(DomainId::new("test..domain").is_err());
    }

    #[test]
    fn builds_empty_catalog() {
        let catalog = DomainExtensionBuilder::<()>::new(domain())
            .build(1, 0)
            .expect("empty catalog is valid");

        assert!(catalog.is_empty());
        assert_eq!(catalog.domain().as_str(), "test.domain");
        assert_eq!(catalog.parser_api_major(), 1);
        assert_eq!(catalog.parser_api_minor(), 0);
    }

    #[test]
    fn rejects_duplicate_extension_ids() {
        let mut builder = DomainExtensionBuilder::<()>::new(domain());

        builder
            .register_owned(TestExtension::new("vendor.example", 0))
            .expect("first registration succeeds");

        let result = builder.register_owned(TestExtension::new("vendor.example", 10));

        assert!(matches!(
            result,
            Err(DomainExtensionError::DuplicateExtension { .. })
        ));
    }

    #[test]
    fn orders_by_priority_then_identifier() {
        let mut builder = DomainExtensionBuilder::<()>::new(domain());

        builder
            .register_owned(TestExtension::new("vendor.zeta", 5))
            .expect("registration succeeds");
        builder
            .register_owned(TestExtension::new("vendor.beta", 5))
            .expect("registration succeeds");
        builder
            .register_owned(TestExtension::new("vendor.alpha", 10))
            .expect("registration succeeds");

        let catalog = builder.build(1, 0).expect("catalog is valid");
        let ids: Vec<_> = catalog
            .iter()
            .map(|entry| entry.id().as_str())
            .collect();

        assert_eq!(
            ids,
            ["vendor.alpha", "vendor.beta", "vendor.zeta"]
        );
    }

    #[test]
    fn rejects_incompatible_parser_api() {
        let mut builder = DomainExtensionBuilder::<()>::new(domain());

        builder
            .register_owned(TestExtension::new("vendor.example", 0))
            .expect("registration succeeds");

        let result = builder.build(2, 0);

        assert!(matches!(
            result,
            Err(DomainExtensionError::IncompatibleParserApi {
                major: 2,
                minor: 0,
                ..
            })
        ));
    }

    #[test]
    fn catalog_lookup_uses_canonical_id() {
        let mut builder = DomainExtensionBuilder::<()>::new(domain());

        builder
            .register_owned(TestExtension::new("vendor.example", 0))
            .expect("registration succeeds");

        let catalog = builder.build(1, 0).expect("catalog is valid");
        let id = ExtensionId::new("vendor.example").expect("valid ID");

        assert!(catalog.get(&id).is_some());
        assert_eq!(catalog.len(), 1);
    }

    #[test]
    fn handler_adapter_preserves_catalog_order() {
        let mut builder = DomainExtensionBuilder::<()>::new(domain());

        builder
            .register_owned(TestExtension::new("vendor.zeta", 1))
            .expect("registration succeeds");
        builder
            .register_owned(TestExtension::new("vendor.alpha", 2))
            .expect("registration succeeds");

        let catalog = builder.build(1, 0).expect("catalog is valid");
        let handlers = catalog.handlers();

        assert_eq!(handlers.len(), 2);
        assert_eq!(handlers[0].descriptor().id().as_str(), "vendor.alpha");
        assert_eq!(handlers[1].descriptor().id().as_str(), "vendor.zeta");
    }
}
