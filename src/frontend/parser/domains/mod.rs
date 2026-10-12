
#![forbid(unsafe_code)]

//! Domain grammar extension composition for Zamani.
//!
//! File: `src/frontend/parser/domains/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide one stable integration boundary for grammar extensions belonging
//! to classical, quantum, HDL, hybrid, and future computational domains.
//!
//! This module composes existing grammar-extension handlers. It does not
//! implement individual domain productions.
//!
//! # Ownership
//!
//! This module owns:
//! - an immutable-after-construction-by-convention collection of registered
//!   domain extension handlers;
//! - deterministic handler ordering;
//! - duplicate extension-ID detection;
//! - forwarding domain parsing to the canonical grammar dispatcher;
//! - a small, stable API for inspecting registered handlers.
//!
//! This module does not own:
//! - tokenization or token definitions;
//! - source spans or source storage;
//! - the canonical AST or an alternative AST;
//! - extension descriptor or extension error definitions;
//! - the canonical extension registry;
//! - core grammar productions;
//! - semantic analysis, type checking, or effect checking;
//! - capability negotiation or resource planning;
//! - quantum operation catalogues or hardware-specific operation lists;
//! - optimization, lowering, scheduling, or runtime execution.
//!
//! # Integration contract
//!
//! - `super::extensions` owns `GrammarExtension`, `ExtensionId`,
//!   `ExtensionPoint`, `ExtensionError`, and `ExtensionParseContext`.
//! - `super::grammar::dispatch_extension` owns extension dispatch policy,
//!   ambiguity checks, compatibility checks, and input-progress checks.
//! - `super::registry` remains responsible for registry integration after its
//!   metadata types have been reconciled with `super::extensions`.
//! - The parser session supplies the active context, lookahead, dialect set,
//!   and parser API version.
//! - Domain implementations preserve syntax needed by the canonical AST
//!   adapter and obey the active parser resource budgets.
//!
//! # Scalability
//!
//! No fixed number of domains, extensions, operations, devices, qubits,
//! processors, nodes, registers, or other computing resources is imposed.
//!
//! The number of handlers is limited by available memory and any applicable
//! parser configuration or application policy. This module does not impose
//! a language-level capacity ceiling.
//!
//! # Extension policy
//!
//! Domain names and extension IDs are data, not exhaustive Rust enum variants.
//! Registering a new domain extension does not require modifying this module.
//!
//! Each extension must declare its supported grammar points, parser API
//! compatibility, dialect requirements, and conflicts through the canonical
//! extension descriptor.
//!
//! This collection rejects duplicate extension IDs. Grammar-point ambiguity,
//! declared conflicts, dialect eligibility, and parser progress are checked
//! by the canonical dispatcher.
//!
//! # Concurrency
//!
//! Registered handlers are shared through `Arc` and must implement the
//! canonical `Send + Sync` extension contract. Per-parse mutable state belongs
//! to the parser session, not to shared handler instances.
//!
//! The collection itself is mutable while being assembled. After setup, share
//! it immutably with parsing sessions. Do not mutate the collection during a
//! parse.

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::frontend::lexer::token_kind::TokenKind;

use super::extensions::{
    ExtensionError,
    ExtensionErrorKind,
    ExtensionId,
    ExtensionParseContext,
    ExtensionPoint,
    GrammarExtension,
};

/// Ordered collection of domain grammar extensions.
///
/// The generic `N` is the syntax-node type selected by the parser integration.
/// It may be a canonical AST node or an existing parser-owned intermediate
/// syntax node; this collection does not define another syntax tree.
///
/// A `BTreeMap` provides deterministic iteration by extension ID, independent
/// of registration order. Dispatch priority and matching ambiguity are still
/// determined by the canonical grammar dispatcher.
pub struct DomainExtensionSet<N> {
    handlers: BTreeMap<ExtensionId, Arc<dyn GrammarExtension<N>>>,
}

impl<N> Default for DomainExtensionSet<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N> DomainExtensionSet<N> {
    /// Create an empty domain-extension collection.
    ///
    /// Built-in and third-party handlers can be registered explicitly by the
    /// application or parser bootstrap layer. This constructor does not assume
    /// that any domain handler has already been implemented.
    #[must_use]
    pub fn new() -> Self {
        Self {
            handlers: BTreeMap::new(),
        }
    }

    /// Register a domain grammar extension.
    ///
    /// # Errors
    ///
    /// Returns `DuplicateExtension` if another handler already uses the same
    /// extension ID. A failed registration leaves the collection unchanged.
    ///
    /// Descriptor validation and parser API compatibility are owned by the
    /// canonical extension and registry layers.
    pub fn register(
        &mut self,
        handler: Arc<dyn GrammarExtension<N>>,
    ) -> Result<(), ExtensionError> {
        let id = handler.descriptor().id().clone();

        if self.handlers.contains_key(&id) {
            return Err(
                ExtensionError::new(
                    ExtensionErrorKind::DuplicateExtension,
                    format!("domain extension {id} is already registered"),
                )
                .with_extension(id),
            );
        }

        self.handlers.insert(id, handler);
        Ok(())
    }

    /// Return whether an extension ID is registered.
    #[must_use]
    pub fn contains(&self, id: &ExtensionId) -> bool {
        self.handlers.contains_key(id)
    }

    /// Return the number of registered handlers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.handlers.len()
    }

    /// Return whether no handlers are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }

    /// Iterate over handlers in deterministic extension-ID order.
    ///
    /// The returned iterator does not permit mutation of the collection.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &Arc<dyn GrammarExtension<N>>> {
        self.handlers.values()
    }

    /// Produce an owned, deterministically ordered list for the dispatcher.
    ///
    /// Cloning an `Arc` does not clone the handler implementation.
    /// The returned list can be passed directly to
    /// `super::grammar::dispatch_extension`.
    #[must_use]
    pub fn handlers_for_dispatch(
        &self,
    ) -> Vec<Arc<dyn GrammarExtension<N>>> {
        self.handlers.values().cloned().collect()
    }

    /// Dispatch a grammar extension at the requested grammar point.
    ///
    /// This is a thin adapter over the canonical dispatcher. It deliberately
    /// does not duplicate extension matching, priority, conflict, compatibility,
    /// or progress logic.
    ///
    /// # Arguments
    ///
    /// - `point`: the current grammar extension point.
    /// - `context`: the active parser session's extension context.
    /// - `lookahead`: token kinds beginning at the current parser position.
    /// - `active_dialects`: dialect IDs enabled for this parse.
    /// - `parser_api`: the active parser API `(major, minor)` version.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(node))` when one eligible extension successfully parses.
    /// - `Ok(None)` when no registered extension matches.
    /// - `Err(error)` for ambiguity, incompatibility, conflicts, malformed
    ///   syntax, resource exhaustion, or other extension failures.
    ///
    /// The caller remains responsible for invoking this method only at
    /// grammar points where extensions are permitted.
    pub fn dispatch(
        &self,
        point: ExtensionPoint,
        context: &mut dyn ExtensionParseContext,
        lookahead: &[TokenKind],
        active_dialects: &std::collections::BTreeSet<String>,
        parser_api: (u64, u64),
    ) -> Result<Option<N>, ExtensionError> {
        let handlers = self.handlers_for_dispatch();

        super::grammar::dispatch_extension(
            &handlers,
            point,
            context,
            lookahead,
            active_dialects,
            parser_api,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::DomainExtensionSet;

    use std::collections::BTreeSet;
    use std::sync::Arc;

    use crate::frontend::lexer::span::Span;
    use crate::frontend::lexer::token_kind::TokenKind;

    use super::super::extensions::{
        ExtensionApiCompatibility,
        ExtensionDescriptor,
        ExtensionError,
        ExtensionId,
        ExtensionMatch,
        ExtensionParseContext,
        ExtensionPoint,
        ExtensionVersion,
        GrammarExtension,
    };

    struct TestExtension {
        descriptor: ExtensionDescriptor,
    }

    impl TestExtension {
        fn new(id: &str) -> Self {
            let extension_id =
                ExtensionId::new(id).expect("test extension ID should be valid");

            let compatibility =
                ExtensionApiCompatibility::new(1, 0, None)
                    .expect("test compatibility should be valid");

            let descriptor = ExtensionDescriptor::new(
                extension_id,
                ExtensionVersion::new(1, 0, 0),
                compatibility,
                [ExtensionPoint::Item],
            )
            .expect("test descriptor should be valid");

            Self { descriptor }
        }
    }

    impl GrammarExtension<()> for TestExtension {
        fn descriptor(&self) -> &ExtensionDescriptor {
            &self.descriptor
        }

        fn probe(
            &self,
            point: ExtensionPoint,
            lookahead: &[TokenKind],
        ) -> ExtensionMatch {
            if point == ExtensionPoint::Item
                && lookahead.first() == Some(&TokenKind::Identifier)
            {
                ExtensionMatch::Match
            } else {
                ExtensionMatch::NoMatch
            }
        }

        fn parse(
            &self,
            _point: ExtensionPoint,
            context: &mut dyn ExtensionParseContext,
        ) -> Result<(), ExtensionError> {
            context.advance()
        }
    }

    #[derive(Default)]
    struct TestContext {
        position: usize,
    }

    impl ExtensionParseContext for TestContext {
        fn position(&self) -> usize {
            self.position
        }

        fn lookahead_kind(&self, offset: usize) -> Option<TokenKind> {
            (offset == 0).then_some(TokenKind::Identifier)
        }

        fn lookahead_lexeme(&self, offset: usize) -> Option<&str> {
            (offset == 0).then_some("example")
        }

        fn lookahead_span(&self, _offset: usize) -> Option<Span> {
            None
        }

        fn advance(&mut self) -> Result<(), ExtensionError> {
            self.position += 1;
            Ok(())
        }
    }

    fn extension(id: &str) -> Arc<dyn GrammarExtension<()>> {
        Arc::new(TestExtension::new(id))
    }

    #[test]
    fn starts_empty() {
        let set = DomainExtensionSet::<()>::new();

        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn registers_and_finds_an_extension() {
        let mut set = DomainExtensionSet::<()>::new();
        let handler = extension("domain.quantum");
        let id = handler.descriptor().id().clone();

        set.register(handler)
            .expect("first registration should succeed");

        assert_eq!(set.len(), 1);
        assert!(!set.is_empty());
        assert!(set.contains(&id));
    }

    #[test]
    fn rejects_duplicate_ids_without_changing_the_collection() {
        let mut set = DomainExtensionSet::<()>::new();

        set.register(extension("domain.quantum"))
            .expect("first registration should succeed");

        let error = set
            .register(extension("domain.quantum"))
            .expect_err("duplicate registration should fail");

        assert_eq!(
            error.kind(),
            super::super::extensions::ExtensionErrorKind::DuplicateExtension
        );
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn iteration_is_ordered_by_extension_id() {
        let mut set = DomainExtensionSet::<()>::new();

        set.register(extension("domain.quantum"))
            .expect("registration should succeed");
        set.register(extension("domain.classical"))
            .expect("registration should succeed");
        set.register(extension("vendor.example"))
            .expect("registration should succeed");

        let ids: Vec<_> = set
            .iter()
            .map(|handler| handler.descriptor().id().as_str())
            .collect();

        assert_eq!(
            ids,
            [
                "domain.classical",
                "domain.quantum",
                "vendor.example",
            ]
        );
    }

    #[test]
    fn dispatch_uses_the_canonical_dispatcher() {
        let mut set = DomainExtensionSet::<()>::new();

        set.register(extension("domain.classical"))
            .expect("registration should succeed");

        let mut context = TestContext::default();
        let dialects = BTreeSet::new();

        let result = set
            .dispatch(
                ExtensionPoint::Item,
                &mut context,
                &[TokenKind::Identifier],
                &dialects,
                (1, 0),
            )
            .expect("dispatch should succeed");

        assert_eq!(result, Some(()));
        assert_eq!(context.position(), 1);
    }

    #[test]
    fn returns_none_when_no_extension_matches() {
        let mut set = DomainExtensionSet::<()>::new();

        set.register(extension("domain.classical"))
            .expect("registration should succeed");

        let mut context = TestContext::default();
        let dialects = BTreeSet::new();

        let result = set
            .dispatch(
                ExtensionPoint::Statement,
                &mut context,
                &[TokenKind::Identifier],
                &dialects,
                (1, 0),
            )
            .expect("non-matching dispatch should succeed");

        assert_eq!(result, None);
        assert_eq!(context.position(), 0);
    }
}
