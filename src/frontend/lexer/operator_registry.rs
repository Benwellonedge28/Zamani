#![forbid(unsafe_code)]

//! Data-driven operator registry for the Zamani lexer.
//!
//! File: src/frontend/lexer/operator_registry.rs
//! Minimum Rust: 1.97
//! Edition: 2021
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Own the mapping from registered operator spellings to canonical
//! `TokenKind` values and recognize the longest registered operator prefix.
//!
//! # Responsibilities
//!
//! This module owns:
//! - operator spelling-to-token-kind definitions;
//! - operator definition metadata;
//! - registration and validation;
//! - atomic batch registration;
//! - deterministic operator enumeration;
//! - longest-prefix (maximal-munch) recognition;
//! - detection of duplicate operator spellings;
//! - a dynamic trie for efficient prefix matching.
//!
//! This module does not own:
//! - the canonical `TokenKind` enum;
//! - source decoding or cursor movement;
//! - keyword recognition;
//! - punctuation ownership policy;
//! - lexical configuration or resource budgets;
//! - token construction or source spans;
//! - parser precedence or associativity;
//! - operator overloading or type checking;
//! - quantum gate catalogs or quantum semantics;
//! - HDL semantics or hardware instruction selection;
//! - capability negotiation, resource allocation, or target selection.
//!
//! # Normative references
//!
//! - `grammar/spec/lexical.md`
//! - `grammar/lexer/operators.md`
//! - `grammar/lexer/operators.g4`
//! - `grammar/lexer/tokens.md`
//!
//! # Integration contract
//!
//! - `token_kind.rs` provides canonical token identities.
//! - `token_metadata.rs` provides shared stability and provenance types.
//! - `keyword_registry.rs` owns keyword spellings.
//! - `dialect.rs` should coordinate approved extension activation and
//!   cross-registry spelling conflicts.
//! - `config.rs` supplies language-version and compatibility policy.
//! - `scanner.rs` calls `match_prefix` at the current source position.
//! - `token.rs` stores the resulting kind, source text, and source span.
//! - `diagnostics.rs` reports invalid input and registration failures.
//! - `recovery.rs` defines scanner recovery after lexical errors.
//! - The parser owns precedence and syntactic interpretation.
//! - Semantic analysis owns operator meaning and type resolution.
//! - Canonical IR and backend layers own lowering and target realization.
//!
//! # Maximal-munch contract
//!
//! If multiple registered spellings are prefixes of the same input,
//! `match_prefix` returns the longest matching spelling.
//!
//! For example, when the corresponding definitions are registered:
//!
//! - `>` and `>=` match `>=` as one operator;
//! - `.` and `..` match `..` as one operator;
//! - `..` and `..=` match `..=` as one operator;
//! - `-` and `->` match `->` as one operator.
//!
//! Registration order must not change the result.
//!
//! # Scalability
//!
//! This module has no language-level maximum for the number of operators,
//! dialects, or spelling length. It does not encode hardware capacities,
//! processor counts, qubit counts, memory capacities, or topology limits.
//!
//! Actual allocation and address-space limits are those of the host process.
//! The registry reports allocation-independent validation errors where
//! possible; ordinary Rust allocation failure behavior is not redefined here.
//!
//! # Extension policy
//!
//! An extension may register a new spelling against an existing canonical
//! operator kind when that mapping is valid for the language specification.
//! A genuinely new token identity requires a coordinated addition to
//! `token_kind.rs` and its associated token metadata and compatibility maps.
//!
//! Operator spellings are not automatically accepted just because they are
//! registered here. Dialect activation and language-version policy are
//! responsible for deciding which registrations are active.
//!
//! # Important existing repository integration issue
//!
//! `grammar/lexer/operators.g4` contains operator forms that the current
//! `TokenKind` does not fully represent, including `...`, `++`, `--`, `?.`,
//! `??`, and compound bitwise assignments. The existing `DoubleColon` kind
//! is also classified as punctuation by `TokenKind::category`, although the
//! grammar assigns `::` to the operator family.
//!
//! This registry deliberately rejects token kinds that are not classified
//! as operators. Resolve those inconsistencies in `token_kind.rs` and the
//! compatibility/metadata mappings before registering the complete grammar
//! operator catalog. Do not silently misclassify a token to bypass validation.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use super::token_kind::TokenKind;
use super::token_metadata::{TokenOrigin, TokenStability};

/// One registered operator spelling.
///
/// The spelling is lexical data. Its meaning is determined by later
/// parser and semantic phases.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorDefinition {
    spelling: String,
    kind: TokenKind,
    stability: TokenStability,
    origin: TokenOrigin,
    description: Option<String>,
}

impl OperatorDefinition {
    /// Creates an operator definition.
    ///
    /// The token kind must be classified as an operator by `TokenKind`.
    /// The spelling is validated again when registered.
    pub fn new(
        spelling: impl Into<String>,
        kind: TokenKind,
    ) -> Result<Self, OperatorRegistryError> {
        let spelling = spelling.into();
        validate_spelling(&spelling)?;

        if !kind.is_operator() {
            return Err(OperatorRegistryError::NonOperatorTokenKind {
                spelling,
                kind,
            });
        }

        Ok(Self {
            spelling,
            kind,
            stability: TokenStability::Stable,
            origin: TokenOrigin::Core,
            description: None,
        })
    }

    /// Sets the lifecycle classification of this definition.
    #[must_use]
    pub fn with_stability(mut self, stability: TokenStability) -> Self {
        self.stability = stability;
        self
    }

    /// Sets the provenance of this definition.
    #[must_use]
    pub fn with_origin(mut self, origin: TokenOrigin) -> Self {
        self.origin = origin;
        self
    }

    /// Sets a non-empty human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, OperatorRegistryError> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(OperatorRegistryError::EmptyDescription {
                spelling: self.spelling.clone(),
            });
        }

        self.description = Some(description);
        Ok(self)
    }

    /// Returns the registered spelling.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.spelling
    }

    /// Returns the canonical token kind.
    #[must_use]
    pub const fn kind(&self) -> TokenKind {
        self.kind
    }

    /// Returns the lifecycle classification.
    #[must_use]
    pub const fn stability(&self) -> TokenStability {
        self.stability
    }

    /// Returns the definition's provenance.
    #[must_use]
    pub fn origin(&self) -> &TokenOrigin {
        &self.origin
    }

    /// Returns the optional human-readable description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

/// A successful longest-prefix operator match.
///
/// `byte_len` is measured in UTF-8 bytes, not Unicode scalar values.
/// This allows the scanner to advance a byte-based source cursor safely
/// when the match starts at a valid UTF-8 boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatorMatch<'a> {
    definition: &'a OperatorDefinition,
    byte_len: usize,
}

impl<'a> OperatorMatch<'a> {
    /// Returns the matched operator definition.
    #[must_use]
    pub const fn definition(self) -> &'a OperatorDefinition {
        self.definition
    }

    /// Returns the matched token kind.
    #[must_use]
    pub const fn kind(self) -> TokenKind {
        self.definition.kind()
    }

    /// Returns the exact registered spelling.
    #[must_use]
    pub fn spelling(self) -> &'a str {
        self.definition.spelling()
    }

    /// Returns the matched length in source bytes.
    #[must_use]
    pub const fn byte_len(self) -> usize {
        self.byte_len
    }
}

/// A trie node used for maximal-munch recognition.
///
/// A terminal stores the spelling, which is resolved through the canonical
/// `BTreeMap` when a match is returned. This avoids maintaining a second
/// independently mutable copy of each definition.
#[derive(Debug, Clone, Default)]
struct TrieNode {
    children: BTreeMap<char, TrieNode>,
    terminal: Option<String>,
}

impl TrieNode {
    fn insert(&mut self, spelling: &str) {
        let mut node = self;

        for character in spelling.chars() {
            node = node.children.entry(character).or_default();
        }

        node.terminal = Some(spelling.to_owned());
    }
}

/// Registry of canonical operator spellings.
///
/// The registry uses:
///
/// - a `BTreeMap` as the canonical definition store and deterministic
///   iteration order;
/// - a trie for longest-prefix matching.
///
/// Prefix overlap is permitted and expected. Exact duplicate spellings
/// are rejected.
#[derive(Debug, Clone, Default)]
pub struct OperatorRegistry {
    entries: BTreeMap<String, OperatorDefinition>,
    trie: TrieNode,
}

impl OperatorRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers one operator definition.
    ///
    /// This operation either inserts a complete definition into both the
    /// canonical store and the trie, or returns an error without changing
    /// either structure.
    pub fn register(
        &mut self,
        definition: OperatorDefinition,
    ) -> Result<(), OperatorRegistryError> {
        validate_spelling(definition.spelling())?;

        if !definition.kind().is_operator() {
            return Err(OperatorRegistryError::NonOperatorTokenKind {
                spelling: definition.spelling.clone(),
                kind: definition.kind(),
            });
        }

        if self.entries.contains_key(definition.spelling()) {
            return Err(OperatorRegistryError::DuplicateSpelling {
                spelling: definition.spelling.clone(),
            });
        }

        // Insert into the trie only after all fallible validation is done.
        // The definition store remains the canonical source of metadata.
        self.trie.insert(definition.spelling());
        self.entries
            .insert(definition.spelling.clone(), definition);

        Ok(())
    }

    /// Registers a batch atomically.
    ///
    /// If any definition is invalid or conflicts with an existing spelling,
    /// the original registry remains unchanged.
    ///
    /// The order of definitions in a successful batch does not affect
    /// longest-match results.
    pub fn register_batch<I>(
        &mut self,
        definitions: I,
    ) -> Result<(), OperatorRegistryError>
    where
        I: IntoIterator<Item = OperatorDefinition>,
    {
        let mut candidate = self.clone();

        for definition in definitions {
            candidate.register(definition)?;
        }

        *self = candidate;
        Ok(())
    }

    /// Finds the longest registered operator prefix at the start of `input`.
    ///
    /// The input must begin at a valid UTF-8 boundary, as Rust `str` slices
    /// always do. The returned length is measured in bytes.
    ///
    /// Returns `None` if no registered operator matches the input prefix.
    #[must_use]
    pub fn match_prefix<'a>(&'a self, input: &str) -> Option<OperatorMatch<'a>> {
        let mut node = &self.trie;
        let mut best: Option<(&str, usize)> = None;

        for (byte_index, character) in input.char_indices() {
            let Some(next) = node.children.get(&character) else {
                break;
            };

            node = next;
            let end = byte_index + character.len_utf8();

            if let Some(spelling) = node.terminal.as_deref() {
                best = Some((spelling, end));
            }
        }

        let (spelling, byte_len) = best?;
        let definition = self.entries.get(spelling)?;

        Some(OperatorMatch {
            definition,
            byte_len,
        })
    }

    /// Looks up an exact registered spelling.
    #[must_use]
    pub fn get(&self, spelling: &str) -> Option<&OperatorDefinition> {
        self.entries.get(spelling)
    }

    /// Returns whether an exact spelling is registered.
    #[must_use]
    pub fn contains_spelling(&self, spelling: &str) -> bool {
        self.entries.contains_key(spelling)
    }

    /// Returns the number of registered spellings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the registry has no definitions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterates over definitions in deterministic spelling order.
    pub fn iter(&self) -> impl Iterator<Item = &OperatorDefinition> {
        self.entries.values()
    }

    /// Validates the registry's canonical entries and trie consistency.
    ///
    /// Intended for tests, startup validation, and dialect activation.
    pub fn validate(&self) -> Result<(), OperatorRegistryError> {
        for (spelling, definition) in &self.entries {
            validate_spelling(spelling)?;

            if spelling != definition.spelling() {
                return Err(OperatorRegistryError::InternalSpellingMismatch {
                    key: spelling.clone(),
                    definition_spelling: definition.spelling.clone(),
                });
            }

            if !definition.kind().is_operator() {
                return Err(OperatorRegistryError::NonOperatorTokenKind {
                    spelling: spelling.clone(),
                    kind: definition.kind(),
                });
            }

            let matched = self.match_prefix(spelling).ok_or_else(|| {
                OperatorRegistryError::TrieInconsistency {
                    spelling: spelling.clone(),
                }
            })?;

            if matched.spelling() != spelling || matched.byte_len() != spelling.len() {
                return Err(OperatorRegistryError::TrieInconsistency {
                    spelling: spelling.clone(),
                });
            }
        }

        Ok(())
    }
}

fn validate_spelling(spelling: &str) -> Result<(), OperatorRegistryError> {
    if spelling.is_empty() {
        return Err(OperatorRegistryError::EmptySpelling);
    }

    if spelling.chars().any(char::is_whitespace) {
        return Err(OperatorRegistryError::WhitespaceInSpelling {
            spelling: spelling.to_owned(),
        });
    }

    if spelling.chars().any(char::is_control) {
        return Err(OperatorRegistryError::ControlCharacterInSpelling {
            spelling: spelling.to_owned(),
        });
    }

    Ok(())
}

/// Errors returned by operator definition and registry operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OperatorRegistryError {
    EmptySpelling,
    WhitespaceInSpelling {
        spelling: String,
    },
    ControlCharacterInSpelling {
        spelling: String,
    },
    EmptyDescription {
        spelling: String,
    },
    DuplicateSpelling {
        spelling: String,
    },
    NonOperatorTokenKind {
        spelling: String,
        kind: TokenKind,
    },
    InternalSpellingMismatch {
        key: String,
        definition_spelling: String,
    },
    TrieInconsistency {
        spelling: String,
    },
}

impl fmt::Display for OperatorRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySpelling => {
                write!(f, "operator spelling must not be empty")
            }
            Self::WhitespaceInSpelling { spelling } => {
                write!(
                    f,
                    "operator spelling {spelling:?} must not contain whitespace"
                )
            }
            Self::ControlCharacterInSpelling { spelling } => {
                write!(
                    f,
                    "operator spelling {spelling:?} must not contain control characters"
                )
            }
            Self::EmptyDescription { spelling } => {
                write!(f, "operator {spelling:?} has an empty description")
            }
            Self::DuplicateSpelling { spelling } => {
                write!(f, "operator spelling {spelling:?} is already registered")
            }
            Self::NonOperatorTokenKind { spelling, kind } => {
                write!(
                    f,
                    "operator spelling {spelling:?} uses non-operator token kind \
                     {}",
                    kind.diagnostic_name()
                )
            }
            Self::InternalSpellingMismatch {
                key,
                definition_spelling,
            } => {
                write!(
                    f,
                    "operator registry key {key:?} does not match definition \
                     spelling {definition_spelling:?}"
                )
            }
            Self::TrieInconsistency { spelling } => {
                write!(
                    f,
                    "operator trie does not consistently recognize {spelling:?}"
                )
            }
        }
    }
}

impl Error for OperatorRegistryError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn definition(spelling: &str, kind: TokenKind) -> OperatorDefinition {
        OperatorDefinition::new(spelling, kind)
            .expect("test operator definition should be valid")
    }

    fn basic_registry() -> OperatorRegistry {
        let mut registry = OperatorRegistry::new();

        registry
            .register_batch([
                definition(">", TokenKind::GreaterThan),
                definition(">=", TokenKind::GreaterThanOrEqual),
                definition(".", TokenKind::Dot),
                definition("..", TokenKind::Range),
                definition("..=", TokenKind::RangeInclusive),
                definition("-", TokenKind::Minus),
                definition("->", TokenKind::ThinArrow),
                definition("+", TokenKind::Plus),
                definition("+=", TokenKind::PlusAssign),
            ])
            .expect("built-in test definitions should register");

        registry
    }

    #[test]
    fn matches_exact_operator() {
        let registry = basic_registry();

        let matched = registry
            .match_prefix("+")
            .expect("plus should match");

        assert_eq!(matched.kind(), TokenKind::Plus);
        assert_eq!(matched.spelling(), "+");
        assert_eq!(matched.byte_len(), 1);
    }

    #[test]
    fn prefers_longest_operator_prefix() {
        let registry = basic_registry();

        let matched = registry
            .match_prefix(">=value")
            .expect("greater-than-or-equal should match");

        assert_eq!(matched.kind(), TokenKind::GreaterThanOrEqual);
        assert_eq!(matched.spelling(), ">=");
        assert_eq!(matched.byte_len(), 2);
    }

    #[test]
    fn handles_overlapping_range_operators() {
        let registry = basic_registry();

        let inclusive = registry
            .match_prefix("..=limit")
            .expect("inclusive range should match");
        assert_eq!(inclusive.kind(), TokenKind::RangeInclusive);
        assert_eq!(inclusive.spelling(), "..=");

        let exclusive = registry
            .match_prefix("..limit")
            .expect("exclusive range should match");
        assert_eq!(exclusive.kind(), TokenKind::Range);
        assert_eq!(exclusive.spelling(), "..");
    }

    #[test]
    fn handles_arrow_and_shorter_minus_prefix() {
        let registry = basic_registry();

        let matched = registry
            .match_prefix("->result")
            .expect("arrow should match");

        assert_eq!(matched.kind(), TokenKind::ThinArrow);
        assert_eq!(matched.byte_len(), 2);
    }

    #[test]
    fn does_not_match_when_no_operator_prefix_exists() {
        let registry = basic_registry();

        assert!(registry.match_prefix("word").is_none());
        assert!(registry.match_prefix("").is_none());
    }

    #[test]
    fn registration_order_does_not_change_matching() {
        let mut first = OperatorRegistry::new();
        first
            .register_batch([
                definition(">", TokenKind::GreaterThan),
                definition(">=", TokenKind::GreaterThanOrEqual),
            ])
            .expect("definitions should register");

        let mut second = OperatorRegistry::new();
        second
            .register_batch([
                definition(">=", TokenKind::GreaterThanOrEqual),
                definition(">", TokenKind::GreaterThan),
            ])
            .expect("definitions should register");

        assert_eq!(
            first.match_prefix(">=").map(OperatorMatch::spelling),
            second.match_prefix(">=").map(OperatorMatch::spelling)
        );
    }

    #[test]
    fn rejects_duplicate_spelling() {
        let mut registry = basic_registry();
        let original_len = registry.len();

        let error = registry
            .register(definition(">=", TokenKind::GreaterThanOrEqual))
            .expect_err("duplicate spelling must fail");

        assert!(matches!(
            error,
            OperatorRegistryError::DuplicateSpelling { .. }
        ));
        assert_eq!(registry.len(), original_len);
    }

    #[test]
    fn failed_batch_is_atomic() {
        let mut registry = basic_registry();
        let original_len = registry.len();

        let result = registry.register_batch([
            definition("**", TokenKind::Star),
            definition(">=", TokenKind::GreaterThanOrEqual),
        ]);

        assert!(matches!(
            result,
            Err(OperatorRegistryError::DuplicateSpelling { .. })
        ));
        assert_eq!(registry.len(), original_len);
        assert!(!registry.contains_spelling("**"));
    }

    #[test]
    fn rejects_empty_spelling() {
        assert!(matches!(
            OperatorDefinition::new("", TokenKind::Plus),
            Err(OperatorRegistryError::EmptySpelling)
        ));
    }

    #[test]
    fn rejects_whitespace_in_spelling() {
        assert!(matches!(
            OperatorDefinition::new("+ =", TokenKind::Plus),
            Err(OperatorRegistryError::WhitespaceInSpelling { .. })
        ));
    }

    #[test]
    fn rejects_non_operator_token_kind() {
        assert!(matches!(
            OperatorDefinition::new("fn", TokenKind::KeywordFn),
            Err(OperatorRegistryError::NonOperatorTokenKind { .. })
        ));
    }

    #[test]
    fn iterates_in_deterministic_spelling_order() {
        let registry = basic_registry();
        let spellings: Vec<&str> =
            registry.iter().map(OperatorDefinition::spelling).collect();

        let mut sorted = spellings.clone();
        sorted.sort_unstable();

        assert_eq!(spellings, sorted);
    }

    #[test]
    fn validates_registry_consistency() {
        let registry = basic_registry();

        assert_eq!(registry.validate(), Ok(()));
    }

    #[test]
    fn reports_utf8_byte_length_for_registered_unicode_operator() {
        // This checks byte accounting only. Whether Unicode operators are
        // part of the language surface remains a grammar/specification choice.
        let mut registry = OperatorRegistry::new();
        registry
            .register(definition("⊕", TokenKind::BitXor))
            .expect("operator kind is valid");

        let matched = registry
            .match_prefix("⊕value")
            .expect("registered Unicode spelling should match");

        assert_eq!(matched.spelling(), "⊕");
        assert_eq!(matched.byte_len(), "⊕".len());
    }
}