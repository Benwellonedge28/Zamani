
#![forbid(unsafe_code)]

//! Data-driven keyword registry for the Zamani lexer.
//!
//! This module owns keyword spelling-to-token-kind registration.
//!
//! It does not own:
//! - the canonical TokenKind enum;
//! - token scanning or maximal-munch behavior;
//! - identifier syntax;
//! - parser productions;
//! - semantic interpretation;
//! - quantum gate catalogs or vendor-specific vocabulary;
//! - hardware discovery or resource allocation.
//!
//! Normative references:
//! - grammar/spec/lexical.md
//! - grammar/lexer/keywords.md
//! - grammar/lexer/keywords.g4
//! - grammar/lexer/tokens.md
//!
//! Integration:
//! - token_kind.rs supplies TokenKind.
//! - token_metadata.rs supplies shared token metadata types.
//! - dialect.rs controls which extensions are active.
//! - scanner.rs consults this registry after identifier scanning.
//! - config.rs supplies language-version and compatibility policy.
//! - diagnostics.rs reports invalid registrations and source errors.
//! - api.rs exposes the supported public lexer interface.
//!
//! Keywords are matched exactly. Case folding is intentionally not performed.
//! New domain vocabulary is registered as data rather than added as a new
//! Rust enum variant for every operation, vendor, or device.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use super::token_kind::TokenKind;
use super::token_metadata::TokenStability;

/// Whether a spelling is reserved unconditionally or recognized contextually.
///
/// Contextual recognition requires the scanner/parser integration to decide
/// when a contextual spelling should be treated as a keyword. The registry
/// itself never guesses the syntactic context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum KeywordClass {
    Reserved,
    Contextual,
}

/// Origin of a registered keyword.
///
/// Domain and dialect names are runtime data, not hard-coded enum variants.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum KeywordOrigin {
    Core,
    Dialect(String),
    Extension(String),
}

/// One registered keyword spelling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordDefinition {
    spelling: String,
    kind: TokenKind,
    class: KeywordClass,
    stability: TokenStability,
    origin: KeywordOrigin,
    canonical_spelling: String,
    description: Option<String>,
}

impl KeywordDefinition {
    /// Creates a definition whose canonical spelling is the supplied spelling.
    pub fn new(
        spelling: impl Into<String>,
        kind: TokenKind,
        class: KeywordClass,
    ) -> Result<Self, KeywordRegistryError> {
        let spelling = spelling.into();
        validate_spelling(&spelling)?;

        Ok(Self {
            canonical_spelling: spelling.clone(),
            spelling,
            kind,
            class,
            stability: TokenStability::Stable,
            origin: KeywordOrigin::Core,
            description: None,
        })
    }

    /// Creates an alias that maps to an existing canonical spelling and kind.
    ///
    /// The canonical spelling must be supplied explicitly. Registration
    /// validates that it is present and maps to the same token kind.
    pub fn alias(
        spelling: impl Into<String>,
        canonical_spelling: impl Into<String>,
        kind: TokenKind,
        class: KeywordClass,
    ) -> Result<Self, KeywordRegistryError> {
        let spelling = spelling.into();
        let canonical_spelling = canonical_spelling.into();

        validate_spelling(&spelling)?;
        validate_spelling(&canonical_spelling)?;

        Ok(Self {
            spelling,
            kind,
            class,
            stability: TokenStability::Stable,
            origin: KeywordOrigin::Core,
            canonical_spelling,
            description: None,
        })
    }

    pub fn with_stability(mut self, stability: TokenStability) -> Self {
        self.stability = stability;
        self
    }

    pub fn with_origin(mut self, origin: KeywordOrigin) -> Self {
        self.origin = origin;
        self
    }

    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, KeywordRegistryError> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(KeywordRegistryError::EmptyDescription {
                spelling: self.spelling.clone(),
            });
        }

        self.description = Some(description);
        Ok(self)
    }

    pub fn spelling(&self) -> &str {
        &self.spelling
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn class(&self) -> KeywordClass {
        self.class
    }

    pub fn stability(&self) -> TokenStability {
        self.stability
    }

    pub fn origin(&self) -> &KeywordOrigin {
        &self.origin
    }

    pub fn canonical_spelling(&self) -> &str {
        &self.canonical_spelling
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn is_reserved(&self) -> bool {
        self.class == KeywordClass::Reserved
    }

    pub fn is_contextual(&self) -> bool {
        self.class == KeywordClass::Contextual
    }
}

/// Configuration for keyword lookup.
///
/// Reserved keywords are always eligible for recognition.
/// Contextual keywords are returned only when explicitly enabled by the
/// caller, allowing scanner/parser integration to preserve context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeywordLookup {
    pub include_contextual: bool,
}

impl Default for KeywordLookup {
    fn default() -> Self {
        Self {
            include_contextual: false,
        }
    }
}

/// A deterministic registry of keyword spellings.
///
/// The registry uses an ordered map so iteration and diagnostics do not depend
/// on randomized hash iteration order.
///
/// A spelling has one definition. Aliases must be explicitly registered and
/// point to an existing canonical spelling with the same token kind.
/// Case sensitivity follows the canonical Zamani lexical specification.
#[derive(Debug, Clone, Default)]
pub struct KeywordRegistry {
    entries: BTreeMap<String, KeywordDefinition>,
}

impl KeywordRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers one keyword definition.
    ///
    /// Alias definitions must point to a canonical definition already present
    /// in this registry. A failed registration leaves the registry unchanged.
    pub fn register(
        &mut self,
        definition: KeywordDefinition,
    ) -> Result<(), KeywordRegistryError> {
        validate_spelling(definition.spelling())?;
        validate_spelling(definition.canonical_spelling())?;

        if self.entries.contains_key(definition.spelling()) {
            return Err(KeywordRegistryError::DuplicateSpelling {
                spelling: definition.spelling.clone(),
            });
        }

        let canonical = self
            .entries
            .get(definition.canonical_spelling())
            .ok_or_else(|| KeywordRegistryError::UnknownCanonicalSpelling {
                spelling: definition.spelling.clone(),
                canonical_spelling: definition.canonical_spelling.clone(),
            })?;

        if canonical.kind() != definition.kind() {
            return Err(KeywordRegistryError::AliasKindMismatch {
                spelling: definition.spelling.clone(),
                canonical_spelling: definition.canonical_spelling.clone(),
            });
        }

        // A canonical entry must identify itself as canonical.
        if definition.spelling() == definition.canonical_spelling()
            && self.entries.contains_key(definition.canonical_spelling())
        {
            return Err(KeywordRegistryError::DuplicateSpelling {
                spelling: definition.spelling.clone(),
            });
        }

        self.entries
            .insert(definition.spelling.clone(), definition);

        Ok(())
    }

    /// Registers multiple definitions atomically.
    ///
    /// If any definition is invalid, the original registry is unchanged.
    /// Register canonical definitions before their aliases within the batch.
    pub fn register_batch<I>(
        &mut self,
        definitions: I,
    ) -> Result<(), KeywordRegistryError>
    where
        I: IntoIterator<Item = KeywordDefinition>,
    {
        let mut candidate = self.clone();

        for definition in definitions {
            candidate.register(definition)?;
        }

        *self = candidate;
        Ok(())
    }

    /// Looks up a reserved keyword or an explicitly enabled contextual keyword.
    pub fn lookup(
        &self,
        spelling: &str,
        options: KeywordLookup,
    ) -> Option<&KeywordDefinition> {
        let definition = self.entries.get(spelling)?;

        match definition.class() {
            KeywordClass::Reserved => Some(definition),
            KeywordClass::Contextual if options.include_contextual => Some(definition),
            KeywordClass::Contextual => None,
        }
    }

    /// Looks up reserved keywords only.
    pub fn lookup_reserved(&self, spelling: &str) -> Option<&KeywordDefinition> {
        self.lookup(
            spelling,
            KeywordLookup {
                include_contextual: false,
            },
        )
    }

    /// Looks up contextual keywords only.
    pub fn lookup_contextual(&self, spelling: &str) -> Option<&KeywordDefinition> {
        self.entries
            .get(spelling)
            .filter(|definition| definition.is_contextual())
    }

    /// Returns the registered definition regardless of keyword class.
    ///
    /// This is useful for diagnostics, tooling, documentation, and validation.
    pub fn get(&self, spelling: &str) -> Option<&KeywordDefinition> {
        self.entries.get(spelling)
    }

    pub fn contains_spelling(&self, spelling: &str) -> bool {
        self.entries.contains_key(spelling)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterates in deterministic spelling order.
    pub fn iter(&self) -> impl Iterator<Item = &KeywordDefinition> {
        self.entries.values()
    }

    /// Iterates over reserved keywords in deterministic spelling order.
    pub fn reserved(&self) -> impl Iterator<Item = &KeywordDefinition> {
        self.entries
            .values()
            .filter(|definition| definition.is_reserved())
    }

    /// Iterates over contextual keywords in deterministic spelling order.
    pub fn contextual(&self) -> impl Iterator<Item = &KeywordDefinition> {
        self.entries
            .values()
            .filter(|definition| definition.is_contextual())
    }

    /// Validates that every alias points to a canonical definition of the
    /// same kind. Useful at dialect activation and build-time conformance.
    pub fn validate(&self) -> Result<(), KeywordRegistryError> {
        for definition in self.entries.values() {
            let canonical = self
                .entries
                .get(definition.canonical_spelling())
                .ok_or_else(|| KeywordRegistryError::UnknownCanonicalSpelling {
                    spelling: definition.spelling.clone(),
                    canonical_spelling: definition.canonical_spelling.clone(),
                })?;

            if canonical.kind() != definition.kind() {
                return Err(KeywordRegistryError::AliasKindMismatch {
                    spelling: definition.spelling.clone(),
                    canonical_spelling: definition.canonical_spelling.clone(),
                });
            }

            if canonical.canonical_spelling() != canonical.spelling() {
                return Err(KeywordRegistryError::InvalidCanonicalDefinition {
                    spelling: canonical.spelling.clone(),
                });
            }
        }

        Ok(())
    }
}

fn validate_spelling(spelling: &str) -> Result<(), KeywordRegistryError> {
    if spelling.is_empty() {
        return Err(KeywordRegistryError::EmptySpelling);
    }

    if spelling.trim() != spelling {
        return Err(KeywordRegistryError::WhitespaceAroundSpelling {
            spelling: spelling.to_owned(),
        });
    }

    Ok(())
}

/// Errors produced while registering or validating keyword definitions.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum KeywordRegistryError {
    EmptySpelling,
    WhitespaceAroundSpelling {
        spelling: String,
    },
    EmptyDescription {
        spelling: String,
    },
    DuplicateSpelling {
        spelling: String,
    },
    UnknownCanonicalSpelling {
        spelling: String,
        canonical_spelling: String,
    },
    AliasKindMismatch {
        spelling: String,
        canonical_spelling: String,
    },
    InvalidCanonicalDefinition {
        spelling: String,
    },
}

impl fmt::Display for KeywordRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySpelling => write!(f, "keyword spelling must not be empty"),
            Self::WhitespaceAroundSpelling { spelling } => write!(
                f,
                "keyword spelling `{spelling}` must not have surrounding whitespace"
            ),
            Self::EmptyDescription { spelling } => {
                write!(f, "keyword `{spelling}` has an empty description")
            }
            Self::DuplicateSpelling { spelling } => {
                write!(f, "keyword spelling `{spelling}` is already registered")
            }
            Self::UnknownCanonicalSpelling {
                spelling,
                canonical_spelling,
            } => write!(
                f,
                "keyword `{spelling}` refers to unregistered canonical spelling \
                 `{canonical_spelling}`"
            ),
            Self::AliasKindMismatch {
                spelling,
                canonical_spelling,
            } => write!(
                f,
                "keyword alias `{spelling}` does not use the token kind of \
                 `{canonical_spelling}`"
            ),
            Self::InvalidCanonicalDefinition { spelling } => write!(
                f,
                "keyword `{spelling}` is not a valid canonical definition"
            ),
        }
    }
}

impl Error for KeywordRegistryError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn reserved(
        spelling: &str,
        kind: TokenKind,
    ) -> KeywordDefinition {
        KeywordDefinition::new(spelling, kind, KeywordClass::Reserved)
            .expect("test spelling should be valid")
    }

    fn contextual(
        spelling: &str,
        kind: TokenKind,
    ) -> KeywordDefinition {
        KeywordDefinition::new(spelling, kind, KeywordClass::Contextual)
            .expect("test spelling should be valid")
    }

    #[test]
    fn registers_and_finds_reserved_keyword() {
        let mut registry = KeywordRegistry::new();

        registry
            .register(reserved("identifier", TokenKind::Identifier))
            .expect("registration should succeed");

        assert_eq!(
            registry
                .lookup_reserved("identifier")
                .map(KeywordDefinition::kind),
            Some(TokenKind::Identifier)
        );
    }

    #[test]
    fn keyword_lookup_is_case_sensitive() {
        let mut registry = KeywordRegistry::new();

        registry
            .register(reserved("example", TokenKind::Identifier))
            .expect("registration should succeed");

        assert!(registry.lookup_reserved("example").is_some());
        assert!(registry.lookup_reserved("Example").is_none());
    }

    #[test]
    fn contextual_keyword_requires_explicit_lookup() {
        let mut registry = KeywordRegistry::new();

        registry
            .register(contextual("measure", TokenKind::Identifier))
            .expect("registration should succeed");

        assert!(registry.lookup_reserved("measure").is_none());
        assert!(registry.lookup_contextual("measure").is_some());
        assert!(registry
            .lookup(
                "measure",
                KeywordLookup {
                    include_contextual: true,
                }
            )
            .is_some());
    }

    #[test]
    fn rejects_duplicate_spelling() {
        let mut registry = KeywordRegistry::new();

        registry
            .register(reserved("example", TokenKind::Identifier))
            .expect("first registration should succeed");

        let result = registry.register(reserved("example", TokenKind::Eof));

        assert!(matches!(
            result,
            Err(KeywordRegistryError::DuplicateSpelling { .. })
        ));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn batch_registration_is_atomic() {
        let mut registry = KeywordRegistry::new();

        let result = registry.register_batch([
            reserved("first", TokenKind::Identifier),
            reserved("first", TokenKind::Eof),
        ]);

        assert!(result.is_err());
        assert!(registry.is_empty());
    }

    #[test]
    fn aliases_must_point_to_an_existing_canonical_definition() {
        let mut registry = KeywordRegistry::new();

        let alias = KeywordDefinition::alias(
            "pub",
            "public",
            TokenKind::Identifier,
            KeywordClass::Reserved,
        )
        .expect("alias shape should be valid");

        let result = registry.register(alias);

        assert!(matches!(
            result,
            Err(KeywordRegistryError::UnknownCanonicalSpelling { .. })
        ));
    }

    #[test]
    fn explicit_alias_uses_the_canonical_kind() {
        let mut registry = KeywordRegistry::new();

        registry
            .register(reserved("public", TokenKind::Identifier))
            .expect("canonical registration should succeed");

        registry
            .register(
                KeywordDefinition::alias(
                    "pub",
                    "public",
                    TokenKind::Identifier,
                    KeywordClass::Reserved,
                )
                .expect("alias construction should succeed"),
            )
            .expect("alias registration should succeed");

        assert_eq!(
            registry.lookup_reserved("pub").map(KeywordDefinition::kind),
            Some(TokenKind::Identifier)
        );
        assert!(registry.validate().is_ok());
    }

    #[test]
    fn rejects_alias_with_different_token_kind() {
        let mut registry = KeywordRegistry::new();

        registry
            .register(reserved("public", TokenKind::Identifier))
            .expect("canonical registration should succeed");

        let result = registry.register(
            KeywordDefinition::alias(
                "pub",
                "public",
                TokenKind::Eof,
                KeywordClass::Reserved,
            )
            .expect("alias construction should succeed"),
        );

        assert!(matches!(
            result,
            Err(KeywordRegistryError::AliasKindMismatch { .. })
        ));
    }

    #[test]
    fn registry_iteration_is_deterministic() {
        let mut registry = KeywordRegistry::new();

        registry
            .register(reserved("zeta", TokenKind::Identifier))
            .expect("registration should succeed");

        registry
            .register(reserved("alpha", TokenKind::Eof))
            .expect("registration should succeed");

        let names: Vec<_> = registry
            .iter()
            .map(KeywordDefinition::spelling)
            .collect();

        assert_eq!(names, vec!["alpha", "zeta"]);
    }

    #[test]
    fn rejects_empty_and_whitespace_padded_spellings() {
        assert!(matches!(
            KeywordDefinition::new("", TokenKind::Identifier, KeywordClass::Reserved),
            Err(KeywordRegistryError::EmptySpelling)
        ));

        assert!(matches!(
            KeywordDefinition::new(
                " example ",
                TokenKind::Identifier,
                KeywordClass::Reserved
            ),
            Err(KeywordRegistryError::WhitespaceAroundSpelling { .. })
        ));
    }
}
