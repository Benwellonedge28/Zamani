
#![forbid(unsafe_code)]

//! Canonical metadata for Zamani lexical token kinds.
//!
//! # Responsibility
//!
//! This module describes token identity, category, stability, provenance,
//! documented spellings, and optional lifecycle information.
//!
//! It does not scan source text, resolve keywords, choose dialects, parse
//! syntax, interpret quantum operations, or select hardware.
//!
//! # Architecture
//!
//! ```text
//! grammar/spec/lexical.md
//!          |
//!          v
//! grammar/lexer/tokens.md
//!          |
//!          v
//! token_kind.rs <---- token_metadata.rs
//!                          |
//!              +-----------+-----------+
//!              |           |           |
//!              v           v           v
//!        keyword_registry operator_registry dialect.rs
//!              |           |           |
//!              +-----------+-----------+
//!                          |
//!                          v
//!                      scanner.rs
//!                          |
//!                          v
//!                       token.rs
//!                          |
//!                          v
//!                  parser / AST adapter
//! ```
//!
//! # Integration contract
//!
//! `TokenKind` is owned by `token_kind.rs`.
//! Metadata registration is owned by this module.
//! Keyword recognition is owned by `keyword_registry.rs`.
//! Operator recognition is owned by `operator_registry.rs`.
//! Dialect-specific registration is owned by `dialect.rs`.
//! Source scanning is owned by `scanner.rs`.
//!
//! Registries that recognize spellings must validate conflicts according
//! to the lexical specification. This module intentionally does not impose
//! a global spelling-uniqueness rule: a spelling may have a documented,
//! explicitly managed contextual interpretation.
//!
//! # Scalability
//!
//! This module imposes no fixed limit on the number of token kinds,
//! dialects, aliases, or documented spellings. Actual memory and address-space
//! constraints remain those of the host process and its available resources.
//!
//! No target capacity, qubit count, processor count, register width, memory
//! capacity, or device count belongs in token metadata.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use super::token_kind::TokenKind;

/// Broad lexical category for a token.
///
/// Categories are descriptive metadata, not parser or semantic decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum TokenCategory {
    Identifier,
    Keyword,
    Operator,
    Punctuation,
    Literal,
    Annotation,
    Comment,
    Whitespace,
    EndOfInput,
    Error,
    Other,
}

/// Lifecycle state of a token definition.
///
/// `Reserved` means a spelling or category is intentionally set aside by
/// the language policy. It does not imply that the scanner must emit it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum TokenStability {
    Stable,
    Experimental,
    Deprecated,
    Reserved,
    Internal,
}

/// Origin of a token definition.
///
/// Dialect and vendor identifiers are data, not new Rust enum variants.
/// This lets additional dialects register metadata without changing this
/// module whenever a new vendor or computational domain is introduced.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum TokenOrigin {
    Core,
    Dialect(String),
    Vendor(String),
    Extension(String),
}

impl Default for TokenOrigin {
    fn default() -> Self {
        Self::Core
    }
}

/// Lifecycle information that can accompany a token definition.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TokenLifecycle {
    /// Language version in which this definition was introduced, if known.
    pub introduced_in: Option<String>,

    /// Language version in which deprecation was declared, if applicable.
    pub deprecated_in: Option<String>,

    /// Canonical replacement name, if the definition is deprecated.
    ///
    /// This is descriptive metadata; it does not rewrite source code.
    pub replacement: Option<String>,
}

/// Metadata associated with one canonical token kind.
///
/// The token kind itself is deliberately not inferred from its spelling.
/// For example, the spelling-to-kind mapping for keywords belongs to the
/// keyword registry, not to this structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenMetadata {
    kind: TokenKind,
    canonical_name: String,
    category: TokenCategory,
    stability: TokenStability,
    origin: TokenOrigin,
    spellings: BTreeSet<String>,
    description: Option<String>,
    contextual: bool,
    parser_visible: bool,
    lifecycle: TokenLifecycle,
}

impl TokenMetadata {
    /// Creates metadata for a token kind.
    ///
    /// The name must be non-empty. Call `validate` before registering
    /// metadata from external configuration or a dialect.
    pub fn new(
        kind: TokenKind,
        canonical_name: impl Into<String>,
        category: TokenCategory,
    ) -> Self {
        Self {
            kind,
            canonical_name: canonical_name.into(),
            category,
            stability: TokenStability::Stable,
            origin: TokenOrigin::Core,
            spellings: BTreeSet::new(),
            description: None,
            contextual: false,
            parser_visible: true,
            lifecycle: TokenLifecycle::default(),
        }
    }

    /// Sets the stability classification.
    pub fn with_stability(mut self, stability: TokenStability) -> Self {
        self.stability = stability;
        self
    }

    /// Sets the provenance of this token definition.
    pub fn with_origin(mut self, origin: TokenOrigin) -> Self {
        self.origin = origin;
        self
    }

    /// Sets a human-readable description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Marks whether recognition depends on lexical/parser context.
    ///
    /// This flag is descriptive. The scanner and the appropriate registry
    /// remain responsible for implementing contextual recognition.
    pub fn with_contextual(mut self, contextual: bool) -> Self {
        self.contextual = contextual;
        self
    }

    /// Sets whether this token is intended to be visible to parser consumers.
    ///
    /// Trivia may be retained for formatting and tooling while being excluded
    /// from the ordinary parser-facing stream.
    pub fn with_parser_visible(mut self, parser_visible: bool) -> Self {
        self.parser_visible = parser_visible;
        self
    }

    /// Sets language-version lifecycle metadata.
    pub fn with_lifecycle(mut self, lifecycle: TokenLifecycle) -> Self {
        self.lifecycle = lifecycle;
        self
    }

    /// Adds a documented source spelling.
    ///
    /// An empty spelling is rejected. Duplicate spellings within the same
    /// definition are harmless and are stored only once.
    pub fn with_spelling(
        mut self,
        spelling: impl Into<String>,
    ) -> Result<Self, TokenMetadataError> {
        let spelling = spelling.into();

        if spelling.is_empty() {
            return Err(TokenMetadataError::EmptySpelling {
                canonical_name: self.canonical_name.clone(),
            });
        }

        self.spellings.insert(spelling);
        Ok(self)
    }

    /// Adds several documented source spellings.
    pub fn with_spellings<I, S>(
        mut self,
        spellings: I,
    ) -> Result<Self, TokenMetadataError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for spelling in spellings {
            let spelling = spelling.into();

            if spelling.is_empty() {
                return Err(TokenMetadataError::EmptySpelling {
                    canonical_name: self.canonical_name.clone(),
                });
            }

            self.spellings.insert(spelling);
        }

        Ok(self)
    }

    /// Validates this metadata record independently of any registry.
    pub fn validate(&self) -> Result<(), TokenMetadataError> {
        if self.canonical_name.trim().is_empty() {
            return Err(TokenMetadataError::EmptyCanonicalName);
        }

        if self
            .description
            .as_ref()
            .is_some_and(|description| description.trim().is_empty())
        {
            return Err(TokenMetadataError::EmptyDescription {
                canonical_name: self.canonical_name.clone(),
            });
        }

        if let Some(version) = &self.lifecycle.introduced_in {
            if version.trim().is_empty() {
                return Err(TokenMetadataError::EmptyLifecycleField {
                    canonical_name: self.canonical_name.clone(),
                    field: "introduced_in",
                });
            }
        }

        if let Some(version) = &self.lifecycle.deprecated_in {
            if version.trim().is_empty() {
                return Err(TokenMetadataError::EmptyLifecycleField {
                    canonical_name: self.canonical_name.clone(),
                    field: "deprecated_in",
                });
            }
        }

        if let Some(replacement) = &self.lifecycle.replacement {
            if replacement.trim().is_empty() {
                return Err(TokenMetadataError::EmptyLifecycleField {
                    canonical_name: self.canonical_name.clone(),
                    field: "replacement",
                });
            }
        }

        if self.stability == TokenStability::Deprecated
            && self.lifecycle.replacement.is_some()
            && self
                .lifecycle
                .replacement
                .as_ref()
                .is_some_and(|replacement| replacement.trim().is_empty())
        {
            return Err(TokenMetadataError::EmptyLifecycleField {
                canonical_name: self.canonical_name.clone(),
                field: "replacement",
            });
        }

        Ok(())
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn canonical_name(&self) -> &str {
        &self.canonical_name
    }

    pub fn category(&self) -> TokenCategory {
        self.category
    }

    pub fn stability(&self) -> TokenStability {
        self.stability
    }

    pub fn origin(&self) -> &TokenOrigin {
        &self.origin
    }

    pub fn spellings(&self) -> impl Iterator<Item = &str> {
        self.spellings.iter().map(String::as_str)
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn is_contextual(&self) -> bool {
        self.contextual
    }

    pub fn is_parser_visible(&self) -> bool {
        self.parser_visible
    }

    pub fn lifecycle(&self) -> &TokenLifecycle {
        &self.lifecycle
    }
}

/// Errors returned while creating or registering token metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TokenMetadataError {
    EmptyCanonicalName,
    EmptySpelling {
        canonical_name: String,
    },
    EmptyDescription {
        canonical_name: String,
    },
    EmptyLifecycleField {
        canonical_name: String,
        field: &'static str,
    },
    DuplicateKind {
        canonical_name: String,
    },
    DuplicateCanonicalName {
        canonical_name: String,
    },
}

impl fmt::Display for TokenMetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCanonicalName => {
                write!(f, "token canonical name must not be empty")
            }
            Self::EmptySpelling { canonical_name } => {
                write!(
                    f,
                    "token `{canonical_name}` has an empty source spelling"
                )
            }
            Self::EmptyDescription { canonical_name } => {
                write!(f, "token `{canonical_name}` has an empty description")
            }
            Self::EmptyLifecycleField {
                canonical_name,
                field,
            } => {
                write!(
                    f,
                    "token `{canonical_name}` has an empty lifecycle field `{field}`"
                )
            }
            Self::DuplicateKind { canonical_name } => {
                write!(
                    f,
                    "a token kind is already registered; duplicate `{canonical_name}`"
                )
            }
            Self::DuplicateCanonicalName { canonical_name } => {
                write!(
                    f,
                    "token canonical name `{canonical_name}` is already registered"
                )
            }
        }
    }
}

impl Error for TokenMetadataError {}

/// Deterministic registry of metadata for canonical token kinds.
///
/// This registry indexes token metadata by kind and canonical name. It does
/// not scan source text and does not replace the keyword/operator registries.
///
/// The ordered maps provide stable iteration independent of hash randomization.
#[derive(Debug, Clone, Default)]
pub struct TokenMetadataRegistry {
    by_kind: BTreeMap<TokenKind, TokenMetadata>,
    by_name: BTreeMap<String, TokenKind>,
}

impl TokenMetadataRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers one metadata record.
    ///
    /// Registration is atomic: an error leaves the registry unchanged.
    pub fn register(
        &mut self,
        metadata: TokenMetadata,
    ) -> Result<(), TokenMetadataError> {
        metadata.validate()?;

        if self.by_kind.contains_key(&metadata.kind) {
            return Err(TokenMetadataError::DuplicateKind {
                canonical_name: metadata.canonical_name.clone(),
            });
        }

        if self.by_name.contains_key(&metadata.canonical_name) {
            return Err(TokenMetadataError::DuplicateCanonicalName {
                canonical_name: metadata.canonical_name.clone(),
            });
        }

        self.by_name
            .insert(metadata.canonical_name.clone(), metadata.kind);
        self.by_kind.insert(metadata.kind, metadata);

        Ok(())
    }

    /// Registers multiple records as one transaction.
    ///
    /// If any record is invalid or conflicts with an existing record, none
    /// of the records in this batch are committed.
    pub fn register_batch<I>(
        &mut self,
        metadata: I,
    ) -> Result<(), TokenMetadataError>
    where
        I: IntoIterator<Item = TokenMetadata>,
    {
        let mut candidate = self.clone();

        for record in metadata {
            candidate.register(record)?;
        }

        *self = candidate;
        Ok(())
    }

    pub fn get(&self, kind: TokenKind) -> Option<&TokenMetadata> {
        self.by_kind.get(&kind)
    }

    pub fn get_by_name(&self, canonical_name: &str) -> Option<&TokenMetadata> {
        let kind = self.by_name.get(canonical_name)?;
        self.by_kind.get(kind)
    }

    pub fn contains_kind(&self, kind: TokenKind) -> bool {
        self.by_kind.contains_key(&kind)
    }

    pub fn contains_name(&self, canonical_name: &str) -> bool {
        self.by_name.contains_key(canonical_name)
    }

    pub fn len(&self) -> usize {
        self.by_kind.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_kind.is_empty()
    }

    /// Iterates in canonical token-kind order.
    pub fn iter(&self) -> impl Iterator<Item = &TokenMetadata> {
        self.by_kind.values()
    }

    /// Iterates metadata filtered by category, preserving deterministic order.
    pub fn by_category(
        &self,
        category: TokenCategory,
    ) -> impl Iterator<Item = &TokenMetadata> {
        self.by_kind
            .values()
            .filter(move |metadata| metadata.category == category)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn metadata(
        kind: TokenKind,
        name: &str,
        category: TokenCategory,
    ) -> TokenMetadata {
        TokenMetadata::new(kind, name, category)
    }

    #[test]
    fn metadata_stores_canonical_identity() {
        let record = metadata(
            TokenKind::Identifier,
            "Identifier",
            TokenCategory::Identifier,
        );

        assert!(record.validate().is_ok());
        assert_eq!(record.kind(), TokenKind::Identifier);
        assert_eq!(record.canonical_name(), "Identifier");
        assert_eq!(record.category(), TokenCategory::Identifier);
        assert_eq!(record.stability(), TokenStability::Stable);
        assert_eq!(record.origin(), &TokenOrigin::Core);
    }

    #[test]
    fn metadata_rejects_empty_canonical_name() {
        let record = metadata(
            TokenKind::Identifier,
            "   ",
            TokenCategory::Identifier,
        );

        assert_eq!(
            record.validate(),
            Err(TokenMetadataError::EmptyCanonicalName)
        );
    }

    #[test]
    fn metadata_rejects_empty_spelling() {
        let result = metadata(
            TokenKind::Identifier,
            "Identifier",
            TokenCategory::Identifier,
        )
        .with_spelling("");

        assert!(matches!(
            result,
            Err(TokenMetadataError::EmptySpelling { .. })
        ));
    }

    #[test]
    fn metadata_deduplicates_repeated_spellings() {
        let record = metadata(
            TokenKind::Identifier,
            "Identifier",
            TokenCategory::Identifier,
        )
        .with_spellings(["name", "name", "identifier"])
        .expect("non-empty spellings should be accepted");

        let spellings: Vec<_> = record.spellings().collect();

        assert_eq!(spellings.len(), 2);
        assert!(spellings.contains(&"name"));
        assert!(spellings.contains(&"identifier"));
    }

    #[test]
    fn registry_retrieves_by_kind_and_name() {
        let mut registry = TokenMetadataRegistry::new();

        registry
            .register(metadata(
                TokenKind::Identifier,
                "Identifier",
                TokenCategory::Identifier,
            ))
            .expect("first registration should succeed");

        assert!(registry.contains_kind(TokenKind::Identifier));
        assert!(registry.contains_name("Identifier"));
        assert_eq!(
            registry.get(TokenKind::Identifier).map(TokenMetadata::canonical_name),
            Some("Identifier")
        );
        assert_eq!(
            registry.get_by_name("Identifier").map(TokenMetadata::kind),
            Some(TokenKind::Identifier)
        );
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn registry_rejects_duplicate_kind_without_mutation() {
        let mut registry = TokenMetadataRegistry::new();

        registry
            .register(metadata(
                TokenKind::Identifier,
                "Identifier",
                TokenCategory::Identifier,
            ))
            .expect("initial registration should succeed");

        let error = registry
            .register(metadata(
                TokenKind::Identifier,
                "AnotherIdentifierName",
                TokenCategory::Identifier,
            ))
            .expect_err("a kind must have one canonical metadata record");

        assert!(matches!(
            error,
            TokenMetadataError::DuplicateKind { .. }
        ));
        assert_eq!(registry.len(), 1);
        assert!(registry.get_by_name("AnotherIdentifierName").is_none());
    }

    #[test]
    fn registry_rejects_duplicate_name_without_mutation() {
        let mut registry = TokenMetadataRegistry::new();

        registry
            .register(metadata(
                TokenKind::Identifier,
                "SharedName",
                TokenCategory::Identifier,
            ))
            .expect("initial registration should succeed");

        let error = registry
            .register(metadata(
                TokenKind::Eof,
                "SharedName",
                TokenCategory::EndOfInput,
            ))
            .expect_err("canonical names must be unique");

        assert!(matches!(
            error,
            TokenMetadataError::DuplicateCanonicalName { .. }
        ));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn batch_registration_is_atomic() {
        let mut registry = TokenMetadataRegistry::new();

        let result = registry.register_batch([
            metadata(
                TokenKind::Identifier,
                "Identifier",
                TokenCategory::Identifier,
            ),
            metadata(
                TokenKind::Eof,
                "Identifier",
                TokenCategory::EndOfInput,
            ),
        ]);

        assert!(result.is_err());
        assert!(registry.is_empty());
    }

    #[test]
    fn registry_iteration_is_deterministic() {
        let mut registry = TokenMetadataRegistry::new();

        registry
            .register(metadata(
                TokenKind::Eof,
                "Eof",
                TokenCategory::EndOfInput,
            ))
            .expect("registration should succeed");

        registry
            .register(metadata(
                TokenKind::Identifier,
                "Identifier",
                TokenCategory::Identifier,
            ))
            .expect("registration should succeed");

        let first: Vec<_> = registry
            .iter()
            .map(TokenMetadata::canonical_name)
            .collect();
        let second: Vec<_> = registry
            .iter()
            .map(TokenMetadata::canonical_name)
            .collect();

        assert_eq!(first, second);
    }

    #[test]
    fn metadata_supports_dialect_provenance() {
        let record = metadata(
            TokenKind::Identifier,
            "CustomOperationName",
            TokenCategory::Identifier,
        )
        .with_origin(TokenOrigin::Dialect("custom-quantum".to_owned()))
        .with_contextual(true)
        .with_parser_visible(true);

        assert_eq!(
            record.origin(),
            &TokenOrigin::Dialect("custom-quantum".to_owned())
        );
        assert!(record.is_contextual());
        assert!(record.is_parser_visible());
    }

    #[test]
    fn metadata_supports_lifecycle_information() {
        let lifecycle = TokenLifecycle {
            introduced_in: Some("1.0".to_owned()),
            deprecated_in: Some("2.0".to_owned()),
            replacement: Some("CanonicalReplacement".to_owned()),
        };

        let record = metadata(
            TokenKind::Identifier,
            "LegacyToken",
            TokenCategory::Other,
        )
        .with_stability(TokenStability::Deprecated)
        .with_lifecycle(lifecycle.clone());

        assert!(record.validate().is_ok());
        assert_eq!(record.lifecycle(), &lifecycle);
        assert_eq!(record.stability(), TokenStability::Deprecated);
    }

    #[test]
    fn category_filtering_is_stable() {
        let mut registry = TokenMetadataRegistry::new();

        registry
            .register(metadata(
                TokenKind::Identifier,
                "Identifier",
                TokenCategory::Identifier,
            ))
            .expect("registration should succeed");

        registry
            .register(metadata(
                TokenKind::Eof,
                "Eof",
                TokenCategory::EndOfInput,
            ))
            .expect("registration should succeed");

        let names: Vec<_> = registry
            .by_category(TokenCategory::Identifier)
            .map(TokenMetadata::canonical_name)
            .collect();

        assert_eq!(names, vec!["Identifier"]);
    }
}
