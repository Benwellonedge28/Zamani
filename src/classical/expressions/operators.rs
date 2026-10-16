
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Classical Expressions — Operator Registry
//!
//! File: `src/classical/expressions/operators.rs`
//!
//! Status: Proposed implementation; repository integration and compilation
//! must pass before this file is marked verified or frozen.
//!
//! ## Purpose
//!
//! Provide a deterministic, extensible mapping from parsed operator syntax
//! to the canonical source-level `OperatorRef` owned by `src/frontend/ast/`.
//!
//! This module is an integration registry. It is not an AST, evaluator,
//! type checker, overload resolver, optimizer, or intermediate representation.
//!
//! ## Ownership
//!
//! Canonical AST ownership:
//! `crate::frontend::ast::node::expressions::expression::OperatorRef`.
//!
//! Grammar ownership:
//! Operator spellings, syntactic positions, precedence, and associativity
//! remain the responsibility of the canonical grammar and parser.
//!
//! Semantic ownership:
//! Operator meaning, operand types, overload resolution, effects, and
//! legality remain the responsibility of semantic analysis.
//!
//! This module owns only:
//! - operator-syntax lookup keys;
//! - the extensible operator registry;
//! - registration conflict validation;
//! - deterministic lookup and registry inspection.
//!
//! ## POCO-REAF and scalability
//!
//! The registry has no language-defined maximum number of operators,
//! dialects, or registered spellings. It introduces no machine, hardware,
//! register-width, tensor-rank, or operand-count limits.
//!
//! Registry memory usage grows with registered entries and is bounded in
//! practice by available resources and any explicit policy imposed by its
//! caller. No implicit truncation or silent replacement is performed.
//!
//! ## Integration contract
//!
//! Parser integration:
//! The parser supplies the spelling, syntactic fixity, and optional dialect
//! context. It remains responsible for tokenization and parsing.
//!
//! AST integration:
//! Registry results are the canonical frontend `OperatorRef`; no parallel
//! operator identity is created.
//!
//! Semantic integration:
//! Semantic analysis consumes the returned identity and determines meaning.
//!
//! Classical integration:
//! Classical expression processing may use this registry to normalize
//! operator syntax before semantic resolution or lowering.
//!
//! Quantum and other domains:
//! The registry does not enumerate domain operations. Namespaced identities
//! can be registered by domain integrations, while `quantum::ir` remains
//! the canonical downstream quantum boundary.
//!
//! ## Freeze rule
//!
//! Do not add precedence, numeric behavior, target capabilities, or execution
//! policy here. Those belong to their respective authoritative subsystems.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::frontend::ast::node::expressions::expression::OperatorRef;

/// Syntactic position in which an operator is used.
///
/// Fixity is syntax metadata, not the operator's semantic meaning.
/// A spelling may be registered independently for different fixities:
/// for example, `-` can be prefix negation or an infix subtraction operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperatorFixity {
    /// Operator appears before its operand or operands.
    Prefix,

    /// Operator appears between its operands.
    Infix,

    /// Operator appears after its operand.
    Postfix,
}

/// Canonical key for a spelling in a syntactic position and dialect.
///
/// An empty dialect identifies the default language operator vocabulary.
/// Non-empty dialect names allow extensions to provide context-specific
/// spellings without changing the universal registry contract.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct OperatorKey {
    dialect: String,
    fixity: OperatorFixity,
    spelling: String,
}

impl OperatorKey {
    fn new(
        dialect: impl Into<String>,
        fixity: OperatorFixity,
        spelling: impl Into<String>,
    ) -> Result<Self, OperatorRegistryError> {
        let dialect = dialect.into();
        let spelling = spelling.into();

        if spelling.is_empty() {
            return Err(OperatorRegistryError::EmptySpelling);
        }

        if spelling.contains('\0') {
            return Err(OperatorRegistryError::InvalidSpelling);
        }

        if dialect.contains('\0') {
            return Err(OperatorRegistryError::InvalidDialect);
        }

        Ok(Self {
            dialect,
            fixity,
            spelling,
        })
    }
}

/// Errors produced by operator registration and lookup.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OperatorRegistryError {
    /// An operator spelling cannot be empty.
    EmptySpelling,

    /// A spelling contains a forbidden NUL character.
    InvalidSpelling,

    /// A dialect identifier contains a forbidden NUL character.
    InvalidDialect,

    /// A spelling is already associated with a different operator identity.
    RegistrationConflict {
        /// Dialect in which the conflict occurred.
        dialect: String,

        /// Syntactic position of the operator.
        fixity: OperatorFixity,

        /// Source spelling.
        spelling: String,

        /// Existing canonical identity.
        existing: String,

        /// Requested canonical identity.
        requested: String,
    },

    /// No matching operator was found.
    UnknownOperator {
        /// Dialect used for lookup.
        dialect: String,

        /// Syntactic position of the operator.
        fixity: OperatorFixity,

        /// Source spelling.
        spelling: String,
    },
}

impl fmt::Display for OperatorRegistryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySpelling => {
                formatter.write_str("operator spelling cannot be empty")
            }
            Self::InvalidSpelling => {
                formatter.write_str("operator spelling contains a NUL character")
            }
            Self::InvalidDialect => {
                formatter.write_str("operator dialect contains a NUL character")
            }
            Self::RegistrationConflict {
                dialect,
                fixity,
                spelling,
                existing,
                requested,
            } => write!(
                formatter,
                "operator registration conflict for {spelling:?} \
                 ({fixity:?}, dialect {dialect:?}): \
                 existing identity {existing:?}, requested {requested:?}"
            ),
            Self::UnknownOperator {
                dialect,
                fixity,
                spelling,
            } => write!(
                formatter,
                "unknown operator {spelling:?} \
                 ({fixity:?}, dialect {dialect:?})"
            ),
        }
    }
}

impl Error for OperatorRegistryError {}

/// Result type for operator-registry operations.
pub type OperatorRegistryResult<T> = Result<T, OperatorRegistryError>;

/// Extensible mapping from parsed operator syntax to canonical AST identities.
///
/// The registry is deliberately instance-owned. It uses no global mutable
/// state, environment lookup, filesystem access, or runtime hardware probing.
///
/// `BTreeMap` provides deterministic key ordering for inspection, testing,
/// and reproducible diagnostics. Lookup and insertion are logarithmic in the
/// number of registered entries.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OperatorRegistry {
    entries: BTreeMap<OperatorKey, OperatorRef>,
}

impl OperatorRegistry {
    /// Creates an empty registry.
    ///
    /// The caller or language integration explicitly registers the operator
    /// spellings supported by its grammar and dialect configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a spelling in the default operator vocabulary.
    ///
    /// Re-registering the same key with the same identity is idempotent.
    /// Registering the same key with a different identity is an error;
    /// this method never silently overwrites an existing registration.
    ///
    /// # Errors
    ///
    /// Returns an error if the spelling is invalid or conflicts with an
    /// existing registration.
    pub fn register(
        &mut self,
        fixity: OperatorFixity,
        spelling: impl Into<String>,
        operator: OperatorRef,
    ) -> OperatorRegistryResult<()> {
        self.register_in_dialect("", fixity, spelling, operator)
    }

    /// Registers an operator spelling in a named dialect.
    ///
    /// A dialect-specific entry takes precedence during dialect-aware lookup.
    /// The default vocabulary is used as a fallback only when no exact
    /// dialect-specific entry exists.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid key or conflicting registration.
    pub fn register_in_dialect(
        &mut self,
        dialect: impl Into<String>,
        fixity: OperatorFixity,
        spelling: impl Into<String>,
        operator: OperatorRef,
    ) -> OperatorRegistryResult<()> {
        let key = OperatorKey::new(dialect, fixity, spelling)?;

        match self.entries.get(&key) {
            Some(existing) if existing == &operator => Ok(()),

            Some(existing) => Err(OperatorRegistryError::RegistrationConflict {
                dialect: key.dialect,
                fixity: key.fixity,
                spelling: key.spelling,
                existing: existing.qualified_name(),
                requested: operator.qualified_name(),
            }),

            None => {
                self.entries.insert(key, operator);
                Ok(())
            }
        }
    }

    /// Resolves an operator using the default vocabulary only.
    ///
    /// Returns `None` when the spelling and fixity are not registered.
    #[must_use]
    pub fn resolve(
        &self,
        fixity: OperatorFixity,
        spelling: &str,
    ) -> Option<&OperatorRef> {
        let key = OperatorKey::new("", fixity, spelling).ok()?;
        self.entries.get(&key)
    }

    /// Resolves an operator in a dialect, falling back to the default
    /// vocabulary when the dialect has no matching entry.
    ///
    /// Exact dialect lookup always takes precedence over the default entry.
    /// A malformed dialect or spelling returns `None`.
    #[must_use]
    pub fn resolve_in_dialect(
        &self,
        dialect: &str,
        fixity: OperatorFixity,
        spelling: &str,
    ) -> Option<&OperatorRef> {
        let exact_key =
            OperatorKey::new(dialect, fixity, spelling).ok()?;

        if let Some(operator) = self.entries.get(&exact_key) {
            return Some(operator);
        }

        if dialect.is_empty() {
            return None;
        }

        let fallback_key =
            OperatorKey::new("", fixity, spelling).ok()?;

        self.entries.get(&fallback_key)
    }

    /// Resolves an operator or returns a structured lookup error.
    ///
    /// # Errors
    ///
    /// Returns `UnknownOperator` when no entry matches the requested key.
    pub fn require(
        &self,
        fixity: OperatorFixity,
        spelling: &str,
    ) -> OperatorRegistryResult<&OperatorRef> {
        self.resolve(fixity, spelling)
            .ok_or_else(|| OperatorRegistryError::UnknownOperator {
                dialect: String::new(),
                fixity,
                spelling: spelling.to_owned(),
            })
    }

    /// Resolves an operator in a dialect or returns a structured error.
    ///
    /// # Errors
    ///
    /// Returns `UnknownOperator` when neither the dialect-specific entry
    /// nor the default vocabulary contains the requested key.
    pub fn require_in_dialect(
        &self,
        dialect: &str,
        fixity: OperatorFixity,
        spelling: &str,
    ) -> OperatorRegistryResult<&OperatorRef> {
        self.resolve_in_dialect(dialect, fixity, spelling)
            .ok_or_else(|| OperatorRegistryError::UnknownOperator {
                dialect: dialect.to_owned(),
                fixity,
                spelling: spelling.to_owned(),
            })
    }

    /// Returns whether the registry contains an exact default-vocabulary key.
    #[must_use]
    pub fn contains(
        &self,
        fixity: OperatorFixity,
        spelling: &str,
    ) -> bool {
        self.resolve(fixity, spelling).is_some()
    }

    /// Returns the number of registered keys.
    ///
    /// This is the actual registry size, not a language-level maximum.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether no entries are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Removes an exact default-vocabulary registration.
    ///
    /// Returns the removed canonical identity, if present.
    pub fn remove(
        &mut self,
        fixity: OperatorFixity,
        spelling: &str,
    ) -> Option<OperatorRef> {
        let key = OperatorKey::new("", fixity, spelling).ok()?;
        self.entries.remove(&key)
    }

    /// Removes an exact dialect-specific registration.
    ///
    /// This does not remove a default-vocabulary entry with the same spelling.
    pub fn remove_from_dialect(
        &mut self,
        dialect: &str,
        fixity: OperatorFixity,
        spelling: &str,
    ) -> Option<OperatorRef> {
        let key = OperatorKey::new(dialect, fixity, spelling).ok()?;
        self.entries.remove(&key)
    }

    /// Iterates over registrations in deterministic key order.
    ///
    /// Each item contains `(dialect, fixity, spelling, operator_identity)`.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&str, OperatorFixity, &str, &OperatorRef)> {
        self.entries.iter().map(|(key, operator)| {
            (
                key.dialect.as_str(),
                key.fixity,
                key.spelling.as_str(),
                operator,
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn operator(name: &str) -> OperatorRef {
        OperatorRef::new("zamani.operator", name)
            .expect("test operator identity should be valid")
    }

    #[test]
    fn registers_and_resolves_a_default_operator() {
        let mut registry = OperatorRegistry::new();

        registry
            .register(OperatorFixity::Infix, "+", operator("add"))
            .expect("registration should succeed");

        assert_eq!(
            registry.resolve(OperatorFixity::Infix, "+"),
            Some(&operator("add"))
        );
    }

    #[test]
    fn identical_registration_is_idempotent() {
        let mut registry = OperatorRegistry::new();

        registry
            .register(OperatorFixity::Infix, "+", operator("add"))
            .expect("first registration should succeed");

        registry
            .register(OperatorFixity::Infix, "+", operator("add"))
            .expect("identical registration should be idempotent");

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn conflicting_registration_does_not_replace_the_existing_operator() {
        let mut registry = OperatorRegistry::new();

        registry
            .register(OperatorFixity::Infix, "+", operator("add"))
            .expect("initial registration should succeed");

        let result =
            registry.register(OperatorFixity::Infix, "+", operator("custom"));

        assert!(matches!(
            result,
            Err(OperatorRegistryError::RegistrationConflict { .. })
        ));

        assert_eq!(
            registry.resolve(OperatorFixity::Infix, "+"),
            Some(&operator("add"))
        );
    }

    #[test]
    fn the_same_spelling_can_have_different_fixities() {
        let mut registry = OperatorRegistry::new();

        registry
            .register(OperatorFixity::Prefix, "-", operator("negate"))
            .expect("prefix registration should succeed");

        registry
            .register(OperatorFixity::Infix, "-", operator("subtract"))
            .expect("infix registration should succeed");

        assert_eq!(
            registry.resolve(OperatorFixity::Prefix, "-"),
            Some(&operator("negate"))
        );

        assert_eq!(
            registry.resolve(OperatorFixity::Infix, "-"),
            Some(&operator("subtract"))
        );
    }

    #[test]
    fn dialect_registration_overrides_default_lookup_for_that_dialect() {
        let mut registry = OperatorRegistry::new();

        registry
            .register(OperatorFixity::Infix, "+", operator("add"))
            .expect("default registration should succeed");

        registry
            .register_in_dialect(
                "example.dialect",
                OperatorFixity::Infix,
                "+",
                operator("combine"),
            )
            .expect("dialect registration should succeed");

        assert_eq!(
            registry.resolve_in_dialect(
                "example.dialect",
                OperatorFixity::Infix,
                "+"
            ),
            Some(&operator("combine"))
        );

        assert_eq!(
            registry.resolve_in_dialect(
                "other.dialect",
                OperatorFixity::Infix,
                "+"
            ),
            Some(&operator("add"))
        );
    }

    #[test]
    fn unknown_operator_is_reported_without_inventing_a_fallback() {
        let registry = OperatorRegistry::new();

        assert!(matches!(
            registry.require(OperatorFixity::Infix, "??"),
            Err(OperatorRegistryError::UnknownOperator { .. })
        ));
    }

    #[test]
    fn invalid_spellings_are_rejected() {
        let mut registry = OperatorRegistry::new();

        assert_eq!(
            registry.register(
                OperatorFixity::Infix,
                "",
                operator("empty")
            ),
            Err(OperatorRegistryError::EmptySpelling)
        );

        assert_eq!(
            registry.register(
                OperatorFixity::Infix,
                "bad\0spelling",
                operator("invalid")
            ),
            Err(OperatorRegistryError::InvalidSpelling)
        );
    }

    #[test]
    fn iteration_is_stable_and_reports_registered_identities() {
        let mut registry = OperatorRegistry::new();

        registry
            .register(OperatorFixity::Infix, "-", operator("subtract"))
            .expect("registration should succeed");

        registry
            .register(OperatorFixity::Infix, "+", operator("add"))
            .expect("registration should succeed");

        let entries: Vec<_> = registry
            .iter()
            .map(|(_, fixity, spelling, operator)| {
                (fixity, spelling.to_owned(), operator.name().to_owned())
            })
            .collect();

        assert_eq!(
            entries,
            vec![
                (OperatorFixity::Infix, "+".to_owned(), "add".to_owned()),
                (
                    OperatorFixity::Infix,
                    "-".to_owned(),
                    "subtract".to_owned()
                ),
            ]
        );
    }

    #[test]
    fn removing_a_dialect_entry_preserves_the_default_entry() {
        let mut registry = OperatorRegistry::new();

        registry
            .register(OperatorFixity::Infix, "+", operator("add"))
            .expect("default registration should succeed");

        registry
            .register_in_dialect(
                "example.dialect",
                OperatorFixity::Infix,
                "+",
                operator("combine"),
            )
            .expect("dialect registration should succeed");

        assert_eq!(
            registry.remove_from_dialect(
                "example.dialect",
                OperatorFixity::Infix,
                "+"
            ),
            Some(operator("combine"))
        );

        assert_eq!(
            registry.resolve(OperatorFixity::Infix, "+"),
            Some(&operator("add"))
        );
    }
}
