
#![forbid(unsafe_code)]

//! Versioned, data-driven dialect registration for the Zamani frontend lexer.
//!
//! File: src/frontend/lexer/dialect.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Define, validate, register, activate, and deactivate lexical dialects
//! without embedding domain-specific vocabulary in the scanner.
//!
//! # Responsibilities
//!
//! This module owns:
//! - dialect identity and version metadata;
//! - dialect definitions and declared dependencies;
//! - deterministic dialect registration;
//! - activation state and dependency validation;
//! - keyword and operator extension coordination;
//! - validation of conflicting lexical spellings;
//! - atomic updates to the keyword and operator registries;
//! - deterministic inspection of registered dialects.
//!
//! This module does not own:
//! - source scanning or cursor movement;
//! - canonical token identities;
//! - core keyword and operator definitions;
//! - parser grammar or precedence;
//! - AST construction or semantic interpretation;
//! - quantum-operation semantics or gate implementations;
//! - HDL synthesis or hardware selection;
//! - target capabilities or resource allocation;
//! - arbitrary code execution by dialect plugins.
//!
//! # Normative references
//!
//! - grammar/DESIGN.md
//! - grammar/specification/lexical.md
//! - grammar/lexer/tokens.md
//! - grammar/lexer/keywords.md
//! - grammar/lexer/operators.md
//! - grammar/Zamani.g4
//!
//! # Integration contract
//!
//! - `token_kind.rs` owns canonical token identities.
//! - `token_metadata.rs` owns token stability and provenance.
//! - `keyword_registry.rs` owns keyword recognition data.
//! - `operator_registry.rs` owns operator recognition data and maximal
//!   munch matching.
//! - `config.rs` owns per-lexer policies and resource budgets.
//! - `limits.rs` owns or re-exports the canonical resource-limit contract.
//! - `scanner.rs` consumes the resolved registries and performs scanning.
//! - `diagnostics.rs` owns source diagnostics.
//! - `recovery.rs` owns malformed-source recovery.
//! - `api.rs` exposes the public lexer interface.
//! - Parser and semantic layers determine syntax and meaning.
//!
//! This module coordinates registration; it does not independently scan
//! source text or introduce a second token vocabulary.
//!
//! # Scalability and portability
//!
//! Dialect names, versions, dependencies, and registered definitions are
//! data. No fixed number of dialects, operators, keywords, devices, qubits,
//! processors, or other computational resources is imposed here.
//!
//! Actual memory, address-space, and execution limits remain host constraints.
//! Registry contents are deterministic and do not depend on registration
//! order for lookup results.
//!
//! # Extension safety
//!
//! A dialect cannot replace an existing keyword or operator definition
//! through this module. A new token kind requires a coordinated change to
//! the canonical token contract, metadata, grammar, parser integration,
//! compatibility mapping, and tests.
//!
//! Dialect registration is not permission to execute plugin code. Any
//! executable extension system must be designed and authorized separately.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use super::keyword_registry::{
    KeywordDefinition, KeywordOrigin, KeywordRegistry, KeywordRegistryError,
};
use super::operator_registry::{
    OperatorDefinition, OperatorRegistry, OperatorRegistryError,
};
use super::token_metadata::{TokenOrigin, TokenStability};

/// Stable, user-defined identity for a lexical dialect.
///
/// Identifiers are case-sensitive. Whitespace-only identifiers are invalid.
/// The identifier is data rather than a Rust enum variant, so new domains
/// and vendor namespaces do not require changes to this module.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DialectId(String);

impl DialectId {
    /// Creates a validated dialect identifier.
    ///
    /// The format is intentionally permissive enough for namespaced,
    /// vendor-defined, and future dialect identifiers.
    pub fn new(value: impl Into<String>) -> Result<Self, DialectError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(DialectError::EmptyIdentifier);
        }

        if value.trim() != value {
            return Err(DialectError::InvalidIdentifier {
                value,
                reason: "leading or trailing whitespace is not allowed",
            });
        }

        if value.chars().any(char::is_control) {
            return Err(DialectError::InvalidIdentifier {
                value,
                reason: "control characters are not allowed",
            });
        }

        Ok(Self(value))
    }

    /// Returns the exact registered identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DialectId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A validated, explicit dialect version.
///
/// The version is an opaque, non-empty string. This module does not assume
/// that every dialect uses semantic versioning. A repository may introduce
/// a shared version parser later without changing dialect identity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DialectVersion(String);

impl DialectVersion {
    /// Creates a dialect version.
    pub fn new(value: impl Into<String>) -> Result<Self, DialectError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(DialectError::EmptyVersion);
        }

        if value.trim() != value || value.chars().any(char::is_control) {
            return Err(DialectError::InvalidVersion { value });
        }

        Ok(Self(value))
    }

    /// Returns the exact version string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DialectVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Classification of a dialect's maturity.
///
/// Maturity is descriptive metadata. It does not prove that a dialect's
/// grammar, semantics, compiler, or runtime is production-ready.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum DialectStability {
    /// Available for general use under the documented compatibility contract.
    Stable,

    /// Subject to change while being developed and evaluated.
    Experimental,

    /// Retained for compatibility but discouraged for new programs.
    Deprecated,
}

impl Default for DialectStability {
    fn default() -> Self {
        Self::Experimental
    }
}

/// Dependency on another registered dialect.
///
/// A dependency is satisfied only when the referenced dialect is registered
/// at the required version and active in the registry state.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DialectDependency {
    id: DialectId,
    version: Option<DialectVersion>,
}

impl DialectDependency {
    /// Requires a dialect without pinning its version.
    ///
    /// The caller must still ensure that the selected version is compatible.
    pub fn any_version(id: DialectId) -> Self {
        Self { id, version: None }
    }

    /// Requires an exact dialect version.
    pub fn exact(id: DialectId, version: DialectVersion) -> Self {
        Self {
            id,
            version: Some(version),
        }
    }

    /// Returns the dependency's dialect identity.
    #[must_use]
    pub fn id(&self) -> &DialectId {
        &self.id
    }

    /// Returns the required version, if pinned.
    #[must_use]
    pub fn version(&self) -> Option<&DialectVersion> {
        self.version.as_ref()
    }
}

/// A complete declarative definition of a lexical dialect.
///
/// Definitions contain data, not executable callbacks. The lexical registry
/// therefore remains deterministic and does not run arbitrary extension code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialectDefinition {
    id: DialectId,
    version: DialectVersion,
    stability: DialectStability,
    description: Option<String>,
    dependencies: BTreeSet<DialectDependency>,
    keywords: Vec<KeywordDefinition>,
    operators: Vec<OperatorDefinition>,
}

impl DialectDefinition {
    /// Creates an empty dialect definition.
    pub fn new(
        id: DialectId,
        version: DialectVersion,
    ) -> Self {
        Self {
            id,
            version,
            stability: DialectStability::Experimental,
            description: None,
            dependencies: BTreeSet::new(),
            keywords: Vec::new(),
            operators: Vec::new(),
        }
    }

    /// Sets the maturity classification.
    #[must_use]
    pub fn with_stability(mut self, stability: DialectStability) -> Self {
        self.stability = stability;
        self
    }

    /// Sets a non-empty description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, DialectError> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(DialectError::EmptyDescription {
                dialect: self.id.clone(),
            });
        }

        self.description = Some(description);
        Ok(self)
    }

    /// Adds a required dialect dependency.
    ///
    /// Duplicate dependency declarations are deduplicated deterministically.
    #[must_use]
    pub fn with_dependency(mut self, dependency: DialectDependency) -> Self {
        self.dependencies.insert(dependency);
        self
    }

    /// Adds a keyword definition.
    ///
    /// Duplicate spellings are detected when the definition is validated
    /// or registered. They are not silently overwritten.
    #[must_use]
    pub fn with_keyword(mut self, keyword: KeywordDefinition) -> Self {
        self.keywords.push(keyword);
        self
    }

    /// Adds an operator definition.
    ///
    /// Duplicate spellings are detected when the definition is validated
    /// or registered. They are not silently overwritten.
    #[must_use]
    pub fn with_operator(mut self, operator: OperatorDefinition) -> Self {
        self.operators.push(operator);
        self
    }

    /// Returns the dialect identifier.
    #[must_use]
    pub fn id(&self) -> &DialectId {
        &self.id
    }

    /// Returns the dialect version.
    #[must_use]
    pub fn version(&self) -> &DialectVersion {
        &self.version
    }

    /// Returns the maturity classification.
    #[must_use]
    pub const fn stability(&self) -> DialectStability {
        self.stability
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Iterates over declared dependencies in deterministic order.
    pub fn dependencies(&self) -> impl Iterator<Item = &DialectDependency> {
        self.dependencies.iter()
    }

    /// Iterates over keyword definitions.
    pub fn keywords(&self) -> impl Iterator<Item = &KeywordDefinition> {
        self.keywords.iter()
    }

    /// Iterates over operator definitions.
    pub fn operators(&self) -> impl Iterator<Item = &OperatorDefinition> {
        self.operators.iter()
    }

    /// Validates dialect-local metadata and spelling uniqueness.
    ///
    /// This does not verify that keywords or operators are compatible with
    /// the active core language. Registry validation performs that check.
    pub fn validate(&self) -> Result<(), DialectError> {
        let mut keyword_spellings = BTreeSet::new();

        for keyword in &self.keywords {
            if !keyword_spellings.insert(keyword.spelling().to_owned()) {
                return Err(DialectError::DuplicateDialectKeyword {
                    dialect: self.id.clone(),
                    spelling: keyword.spelling().to_owned(),
                });
            }
        }

        let mut operator_spellings = BTreeSet::new();

        for operator in &self.operators {
            if !operator_spellings.insert(operator.spelling().to_owned()) {
                return Err(DialectError::DuplicateDialectOperator {
                    dialect: self.id.clone(),
                    spelling: operator.spelling().to_owned(),
                });
            }
        }

        for spelling in &keyword_spellings {
            if operator_spellings.contains(spelling) {
                return Err(DialectError::CrossCategoryConflict {
                    dialect: self.id.clone(),
                    spelling: spelling.clone(),
                });
            }
        }

        if self
            .description
            .as_ref()
            .is_some_and(|description| description.trim().is_empty())
        {
            return Err(DialectError::EmptyDescription {
                dialect: self.id.clone(),
            });
        }

        Ok(())
    }
}

/// The current state of a dialect registration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialectState {
    /// Registered but not enabled for lexical recognition.
    Registered,

    /// Enabled for use by a lexer configured with the resolved registries.
    Active,

    /// Previously active but explicitly disabled.
    Inactive,
}

/// A registered dialect and its activation state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisteredDialect {
    definition: DialectDefinition,
    state: DialectState,
}

impl RegisteredDialect {
    /// Returns the dialect definition.
    #[must_use]
    pub fn definition(&self) -> &DialectDefinition {
        &self.definition
    }

    /// Returns the current state.
    #[must_use]
    pub const fn state(&self) -> DialectState {
        self.state
    }

    /// Returns whether this dialect is active.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        matches!(self.state, DialectState::Active)
    }
}

/// Deterministic registry of dialect definitions.
///
/// This registry manages lifecycle and dependency state. Keyword and operator
/// recognition remain owned by their respective registries.
#[derive(Debug, Clone, Default)]
pub struct DialectRegistry {
    entries: BTreeMap<DialectId, RegisteredDialect>,
}

impl DialectRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a dialect without activating it.
    ///
    /// Registration fails if an identifier already exists. A new version
    /// must use an explicit replacement or version-selection workflow rather
    /// than silently replacing the existing definition.
    pub fn register(
        &mut self,
        definition: DialectDefinition,
    ) -> Result<(), DialectError> {
        definition.validate()?;

        if self.entries.contains_key(definition.id()) {
            return Err(DialectError::DuplicateDialect {
                id: definition.id().clone(),
            });
        }

        let id = definition.id().clone();

        self.entries.insert(
            id,
            RegisteredDialect {
                definition,
                state: DialectState::Registered,
            },
        );

        Ok(())
    }

    /// Registers a batch atomically.
    ///
    /// If any definition fails validation or duplicates an existing
    /// identifier, the original registry is unchanged.
    pub fn register_batch<I>(
        &mut self,
        definitions: I,
    ) -> Result<(), DialectError>
    where
        I: IntoIterator<Item = DialectDefinition>,
    {
        let mut candidate = self.clone();

        for definition in definitions {
            candidate.register(definition)?;
        }

        *self = candidate;
        Ok(())
    }

    /// Looks up a registered dialect.
    #[must_use]
    pub fn get(&self, id: &DialectId) -> Option<&RegisteredDialect> {
        self.entries.get(id)
    }

    /// Iterates through registered dialects in identifier order.
    pub fn iter(&self) -> impl Iterator<Item = &RegisteredDialect> {
        self.entries.values()
    }

    /// Returns the number of registered dialects.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether no dialects are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Activates a registered dialect after checking its dependencies.
    ///
    /// This changes lifecycle state only. It does not mutate a scanner that
    /// has already been constructed. Resolve registries and construct or
    /// reconfigure the scanner after activation.
    pub fn activate(&mut self, id: &DialectId) -> Result<(), DialectError> {
        let entry = self
            .entries
            .get(id)
            .ok_or_else(|| DialectError::UnknownDialect { id: id.clone() })?;

        for dependency in entry.definition.dependencies() {
            let required = self.entries.get(dependency.id()).ok_or_else(|| {
                DialectError::MissingDependency {
                    dialect: id.clone(),
                    dependency: dependency.id().clone(),
                }
            })?;

            if required.state != DialectState::Active {
                return Err(DialectError::InactiveDependency {
                    dialect: id.clone(),
                    dependency: dependency.id().clone(),
                });
            }

            if let Some(expected) = dependency.version() {
                if required.definition.version() != expected {
                    return Err(DialectError::DependencyVersionMismatch {
                        dialect: id.clone(),
                        dependency: dependency.id().clone(),
                        expected: expected.clone(),
                        actual: required.definition.version().clone(),
                    });
                }
            }
        }

        self.entries
            .get_mut(id)
            .expect("entry was checked above")
            .state = DialectState::Active;

        Ok(())
    }

    /// Deactivates a dialect unless another active dialect depends on it.
    pub fn deactivate(&mut self, id: &DialectId) -> Result<(), DialectError> {
        if !self.entries.contains_key(id) {
            return Err(DialectError::UnknownDialect { id: id.clone() });
        }

        for entry in self.entries.values() {
            if entry.state == DialectState::Active
                && entry
                    .definition
                    .dependencies()
                    .any(|dependency| dependency.id() == id)
            {
                return Err(DialectError::RequiredByActiveDialect {
                    id: id.clone(),
                    dependent: entry.definition.id().clone(),
                });
            }
        }

        self.entries
            .get_mut(id)
            .expect("entry was checked above")
            .state = DialectState::Inactive;

        Ok(())
    }

    /// Returns all active dialect definitions in deterministic order.
    pub fn active(&self) -> impl Iterator<Item = &DialectDefinition> {
        self.entries
            .values()
            .filter(|entry| entry.is_active())
            .map(|entry| &entry.definition)
    }

    /// Builds candidate keyword and operator registries with every active
    /// dialect applied.
    ///
    /// The caller supplies the canonical core registries. The method clones
    /// both, applies all active definitions, and returns the pair only if
    /// every registration succeeds. The supplied registries are never
    /// mutated, so a failure cannot leave half-applied extensions.
    ///
    /// Cross-category spelling conflicts are rejected. A project that
    /// deliberately supports contextual lexical overlap must model and
    /// validate that behavior explicitly rather than bypassing this check.
    pub fn resolve(
        &self,
        core_keywords: &KeywordRegistry,
        core_operators: &OperatorRegistry,
    ) -> Result<(KeywordRegistry, OperatorRegistry), DialectError> {
        let mut keywords = core_keywords.clone();
        let mut operators = core_operators.clone();

        for entry in self.entries.values().filter(|entry| entry.is_active()) {
            let definition = &entry.definition;

            for keyword in definition.keywords() {
                if keywords.contains_spelling(keyword.spelling()) {
                    return Err(DialectError::KeywordConflict {
                        dialect: definition.id().clone(),
                        spelling: keyword.spelling().to_owned(),
                    });
                }

                if operators
                    .definitions()
                    .any(|operator| operator.spelling() == keyword.spelling())
                {
                    return Err(DialectError::CrossCategoryConflict {
                        dialect: definition.id().clone(),
                        spelling: keyword.spelling().to_owned(),
                    });
                }
            }

            for operator in definition.operators() {
                if operators
                    .definitions()
                    .any(|existing| existing.spelling() == operator.spelling())
                {
                    return Err(DialectError::OperatorConflict {
                        dialect: definition.id().clone(),
                        spelling: operator.spelling().to_owned(),
                    });
                }

                if keywords.contains_spelling(operator.spelling()) {
                    return Err(DialectError::CrossCategoryConflict {
                        dialect: definition.id().clone(),
                        spelling: operator.spelling().to_owned(),
                    });
                }
            }

            let dialect_name = definition.id().as_str().to_owned();

            let dialect_keywords = definition.keywords().cloned().map(|keyword| {
                keyword.with_origin(KeywordOrigin::Dialect(dialect_name.clone()))
            });

            keywords.register_batch(dialect_keywords)?;

            let dialect_operators = definition.operators().cloned().map(|operator| {
                operator.with_origin(TokenOrigin::Dialect(dialect_name.clone()))
            });

            operators.register_batch(dialect_operators)?;
        }

        Ok((keywords, operators))
    }
}

/// Dialect registration, validation, and activation errors.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DialectError {
    EmptyIdentifier,
    InvalidIdentifier {
        value: String,
        reason: &'static str,
    },
    EmptyVersion,
    InvalidVersion {
        value: String,
    },
    EmptyDescription {
        dialect: DialectId,
    },
    DuplicateDialect {
        id: DialectId,
    },
    UnknownDialect {
        id: DialectId,
    },
    DuplicateDialectKeyword {
        dialect: DialectId,
        spelling: String,
    },
    DuplicateDialectOperator {
        dialect: DialectId,
        spelling: String,
    },
    KeywordConflict {
        dialect: DialectId,
        spelling: String,
    },
    OperatorConflict {
        dialect: DialectId,
        spelling: String,
    },
    CrossCategoryConflict {
        dialect: DialectId,
        spelling: String,
    },
    MissingDependency {
        dialect: DialectId,
        dependency: DialectId,
    },
    InactiveDependency {
        dialect: DialectId,
        dependency: DialectId,
    },
    DependencyVersionMismatch {
        dialect: DialectId,
        dependency: DialectId,
        expected: DialectVersion,
        actual: DialectVersion,
    },
    RequiredByActiveDialect {
        id: DialectId,
        dependent: DialectId,
    },
    KeywordRegistry(String),
    OperatorRegistry(String),
}

impl fmt::Display for DialectError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                formatter.write_str("dialect identifier must not be empty")
            }
            Self::InvalidIdentifier { value, reason } => {
                write!(formatter, "invalid dialect identifier `{value}`: {reason}")
            }
            Self::EmptyVersion => {
                formatter.write_str("dialect version must not be empty")
            }
            Self::InvalidVersion { value } => {
                write!(formatter, "invalid dialect version `{value}`")
            }
            Self::EmptyDescription { dialect } => {
                write!(formatter, "dialect `{dialect}` has an empty description")
            }
            Self::DuplicateDialect { id } => {
                write!(formatter, "dialect `{id}` is already registered")
            }
            Self::UnknownDialect { id } => {
                write!(formatter, "dialect `{id}` is not registered")
            }
            Self::DuplicateDialectKeyword { dialect, spelling } => {
                write!(formatter, "dialect `{dialect}` repeats keyword `{spelling}`")
            }
            Self::DuplicateDialectOperator { dialect, spelling } => {
                write!(formatter, "dialect `{dialect}` repeats operator `{spelling}`")
            }
            Self::KeywordConflict { dialect, spelling } => {
                write!(formatter, "dialect `{dialect}` conflicts with keyword `{spelling}`")
            }
            Self::OperatorConflict { dialect, spelling } => {
                write!(formatter, "dialect `{dialect}` conflicts with operator `{spelling}`")
            }
            Self::CrossCategoryConflict { dialect, spelling } => {
                write!(
                    formatter,
                    "dialect `{dialect}` assigns `{spelling}` to conflicting lexical categories"
                )
            }
            Self::MissingDependency { dialect, dependency } => {
                write!(formatter, "dialect `{dialect}` requires unregistered dialect `{dependency}`")
            }
            Self::InactiveDependency { dialect, dependency } => {
                write!(formatter, "dialect `{dialect}` requires inactive dialect `{dependency}`")
            }
            Self::DependencyVersionMismatch {
                dialect,
                dependency,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "dialect `{dialect}` requires `{dependency}` version `{expected}`, but `{actual}` is registered"
                )
            }
            Self::RequiredByActiveDialect { id, dependent } => {
                write!(
                    formatter,
                    "cannot deactivate `{id}` while active dialect `{dependent}` depends on it"
                )
            }
            Self::KeywordRegistry(message) => {
                write!(formatter, "keyword registry rejected dialect: {message}")
            }
            Self::OperatorRegistry(message) => {
                write!(formatter, "operator registry rejected dialect: {message}")
            }
        }
    }
}

impl Error for DialectError {}

impl From<KeywordRegistryError> for DialectError {
    fn from(error: KeywordRegistryError) -> Self {
        Self::KeywordRegistry(error.to_string())
    }
}

impl From<OperatorRegistryError> for DialectError {
    fn from(error: OperatorRegistryError) -> Self {
        Self::OperatorRegistry(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DialectDefinition, DialectDependency, DialectError, DialectId,
        DialectRegistry, DialectStability, DialectState, DialectVersion,
    };

    fn id(value: &str) -> DialectId {
        DialectId::new(value).expect("valid dialect identifier")
    }

    fn version(value: &str) -> DialectVersion {
        DialectVersion::new(value).expect("valid dialect version")
    }

    fn definition(name: &str) -> DialectDefinition {
        DialectDefinition::new(id(name), version("1.0.0"))
            .with_stability(DialectStability::Stable)
    }

    #[test]
    fn dialect_identifiers_reject_empty_and_trimmed_values() {
        assert!(matches!(
            DialectId::new("  "),
            Err(DialectError::EmptyIdentifier)
        ));

        assert!(matches!(
            DialectId::new(" invalid "),
            Err(DialectError::InvalidIdentifier { .. })
        ));

        assert!(DialectId::new("vendor.quantum").is_ok());
    }

    #[test]
    fn dialect_versions_must_be_nonempty() {
        assert!(DialectVersion::new("").is_err());
        assert!(DialectVersion::new("1.0.0").is_ok());
    }

    #[test]
    fn registration_is_deterministic_and_rejects_duplicates() {
        let mut registry = DialectRegistry::new();
        let dialect = definition("example");

        registry.register(dialect.clone()).expect("first registration");

        assert!(matches!(
            registry.register(dialect),
            Err(DialectError::DuplicateDialect { .. })
        ));

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn batch_registration_is_atomic() {
        let mut registry = DialectRegistry::new();

        registry
            .register(definition("existing"))
            .expect("initial registration");

        let result = registry.register_batch([
            definition("new"),
            definition("existing"),
        ]);

        assert!(matches!(result, Err(DialectError::DuplicateDialect { .. })));
        assert_eq!(registry.len(), 1);
        assert!(registry.get(&id("new")).is_none());
    }

    #[test]
    fn dependency_must_be_registered_and_active() {
        let mut registry = DialectRegistry::new();

        let dependent = definition("dependent")
            .with_dependency(DialectDependency::any_version(id("base")));

        registry.register(dependent).expect("register dependent");

        assert!(matches!(
            registry.activate(&id("dependent")),
            Err(DialectError::MissingDependency { .. })
        ));

        registry
            .register(definition("base"))
            .expect("register base");

        assert!(matches!(
            registry.activate(&id("dependent")),
            Err(DialectError::InactiveDependency { .. })
        ));

        registry.activate(&id("base")).expect("activate base");
        registry
            .activate(&id("dependent"))
            .expect("activate dependent");

        assert_eq!(
            registry.get(&id("dependent")).map(|entry| entry.state()),
            Some(DialectState::Active)
        );
    }

    #[test]
    fn active_dependencies_prevent_unsafe_deactivation() {
        let mut registry = DialectRegistry::new();

        registry
            .register(definition("base"))
            .expect("register base");

        registry
            .register(
                definition("dependent")
                    .with_dependency(DialectDependency::any_version(id("base"))),
            )
            .expect("register dependent");

        registry.activate(&id("base")).expect("activate base");
        registry.activate(&id("dependent")).expect("activate dependent");

        assert!(matches!(
            registry.deactivate(&id("base")),
            Err(DialectError::RequiredByActiveDialect { .. })
        ));
    }

    #[test]
    fn inactive_dialects_are_not_resolved() {
        let registry = DialectRegistry::new();

        assert_eq!(registry.active().count(), 0);
        assert!(registry.is_empty());
    }

    #[test]
    fn definitions_reject_duplicate_local_spellings() {
        let dialect = definition("duplicate");
        assert!(dialect.validate().is_ok());
    }
}
