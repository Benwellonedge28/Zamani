
#![forbid(unsafe_code)]

//! Versioned, data-driven lexical dialect management for Zamani.
//!
//! File: src/frontend/lexer/dialect.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Responsibilities
//!
//! - Define and validate dialect identities, versions, and metadata.
//! - Register dialect definitions atomically.
//! - Validate dependency activation and exact version requirements.
//! - Resolve active dialect extensions into canonical keyword and operator
//!   registries without changing the supplied core registries.
//! - Detect spelling conflicts deterministically.
//! - Keep lexical extensions separate from parsing, semantic analysis,
//!   execution, hardware discovery, and resource allocation.
//!
//! # Architectural boundaries
//!
//! `token_kind.rs` owns canonical token identities.
//! `token_metadata.rs` owns token provenance and stability metadata.
//! `keyword_registry.rs` owns keyword definitions.
//! `operator_registry.rs` owns operator definitions and maximal-munch matching.
//! `config.rs` owns lexer configuration.
//! `scanner.rs` consumes resolved registries.
//! `diagnostics.rs` owns source diagnostics.
//! `recovery.rs` owns lexical recovery.
//! The grammar and parser own syntactic interpretation.
//!
//! Dialects contain declarative data only. This module does not execute
//! plugins, introduce hardware-specific limits, or select a compilation
//! target.
//!
//! No finite language-level limit is imposed on dialect counts, vocabulary
//! counts, or computational resources. Host memory and address-space limits
//! still apply.

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

/// A validated, case-sensitive dialect identifier.
///
/// Namespaced identifiers such as `vendor.quantum` and `vendor.hdl` are
/// permitted. Identifiers are opaque data, not hard-coded Rust variants.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DialectId(String);

impl DialectId {
    /// Creates a dialect identifier.
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

    /// Returns the exact identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DialectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// An opaque, non-empty dialect version.
///
/// Version ordering and compatibility are deliberately not inferred from
/// strings. Dependencies can request an exact version.
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Descriptive maturity metadata.
///
/// This classification does not independently certify grammar,
/// implementation, compiler, or runtime readiness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[non_exhaustive]
pub enum DialectStability {
    /// Intended for general use under a documented compatibility contract.
    Stable,

    /// Subject to change.
    #[default]
    Experimental,

    /// Retained for compatibility but discouraged for new programs.
    Deprecated,
}

/// A dependency on another registered dialect.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DialectDependency {
    id: DialectId,
    version: Option<DialectVersion>,
}

impl DialectDependency {
    /// Requires the dependency to be registered and active.
    ///
    /// No version compatibility is implied by this constructor.
    pub fn any_version(id: DialectId) -> Self {
        Self { id, version: None }
    }

    /// Requires the dependency to be registered and active at this exact
    /// version.
    pub fn exact(id: DialectId, version: DialectVersion) -> Self {
        Self {
            id,
            version: Some(version),
        }
    }

    /// Returns the dependency identity.
    #[must_use]
    pub fn id(&self) -> &DialectId {
        &self.id
    }

    /// Returns the required exact version, if any.
    #[must_use]
    pub fn version(&self) -> Option<&DialectVersion> {
        self.version.as_ref()
    }
}

/// Complete declarative definition of a lexical dialect.
///
/// Definitions may extend existing canonical token kinds. A genuinely new
/// token identity requires coordinated changes to the token contract,
/// metadata, grammar, parser integration, and compatibility mappings.
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
    #[must_use]
    pub fn new(id: DialectId, version: DialectVersion) -> Self {
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

    /// Adds a dependency.
    ///
    /// Identical dependency declarations are deduplicated. Conflicting
    /// version requirements for the same dependency are rejected by
    /// `validate`.
    #[must_use]
    pub fn with_dependency(mut self, dependency: DialectDependency) -> Self {
        self.dependencies.insert(dependency);
        self
    }

    /// Adds a keyword definition.
    ///
    /// Duplicate spellings are rejected by `validate`.
    #[must_use]
    pub fn with_keyword(mut self, keyword: KeywordDefinition) -> Self {
        self.keywords.push(keyword);
        self
    }

    /// Adds an operator definition.
    ///
    /// Duplicate spellings are rejected by `validate`.
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

    /// Iterates over dependencies in deterministic order.
    pub fn dependencies(&self) -> impl Iterator<Item = &DialectDependency> {
        self.dependencies.iter()
    }

    /// Iterates over keyword definitions in declaration order.
    pub fn keywords(&self) -> impl Iterator<Item = &KeywordDefinition> {
        self.keywords.iter()
    }

    /// Iterates over operator definitions in declaration order.
    pub fn operators(&self) -> impl Iterator<Item = &OperatorDefinition> {
        self.operators.iter()
    }

    /// Validates dialect-local invariants.
    ///
    /// Conflicts against the core registries are checked by `resolve`.
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

        // The same dependency may not require two different exact versions.
        let mut dependency_versions: BTreeMap<&DialectId, Option<&DialectVersion>> =
            BTreeMap::new();

        for dependency in &self.dependencies {
            match dependency_versions.get(dependency.id()) {
                Some(existing) if *existing != dependency.version() => {
                    return Err(DialectError::ConflictingDependencyVersions {
                        dialect: self.id.clone(),
                        dependency: dependency.id().clone(),
                    });
                }
                Some(_) => {}
                None => {
                    dependency_versions.insert(dependency.id(), dependency.version());
                }
            }
        }

        Ok(())
    }
}

/// Lifecycle state of a registered dialect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialectState {
    /// Registered but not activated.
    Registered,

    /// Activated and eligible for registry resolution.
    Active,

    /// Explicitly deactivated.
    Inactive,
}

/// A dialect definition and its current lifecycle state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisteredDialect {
    definition: DialectDefinition,
    state: DialectState,
}

impl RegisteredDialect {
    /// Returns the definition.
    #[must_use]
    pub fn definition(&self) -> &DialectDefinition {
        &self.definition
    }

    /// Returns the lifecycle state.
    #[must_use]
    pub const fn state(&self) -> DialectState {
        self.state
    }

    /// Returns whether the dialect is active.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        matches!(self.state, DialectState::Active)
    }
}

/// Deterministic registry of dialect definitions.
///
/// The `BTreeMap` provides stable enumeration independent of registration
/// order. Mutation requires exclusive access, so a caller can construct a
/// candidate configuration before publishing it to lexer instances.
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
    /// Existing registrations are never silently replaced.
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
    /// On any error, the original registry remains unchanged.
    pub fn register_batch<I>(&mut self, definitions: I) -> Result<(), DialectError>
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

    /// Returns a registered dialect by identity.
    #[must_use]
    pub fn get(&self, id: &DialectId) -> Option<&RegisteredDialect> {
        self.entries.get(id)
    }

    /// Iterates over all dialects in identifier order.
    pub fn iter(&self) -> impl Iterator<Item = &RegisteredDialect> {
        self.entries.values()
    }

    /// Returns the number of registered dialects.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Activates a dialect if all declared dependencies are active and
    /// all exact version requirements are satisfied.
    ///
    /// Activation is atomic: state changes only after every dependency
    /// has been validated.
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

            if !required.is_active() {
                return Err(DialectError::InactiveDependency {
                    dialect: id.clone(),
                    dependency: dependency.id().clone(),
                });
            }

            if let Some(expected) = dependency.version() {
                let actual = required.definition.version();

                if actual != expected {
                    return Err(DialectError::DependencyVersionMismatch {
                        dialect: id.clone(),
                        dependency: dependency.id().clone(),
                        expected: expected.clone(),
                        actual: actual.clone(),
                    });
                }
            }
        }

        let entry = self
            .entries
            .get_mut(id)
            .ok_or_else(|| DialectError::UnknownDialect { id: id.clone() })?;

        entry.state = DialectState::Active;
        Ok(())
    }

    /// Deactivates a dialect unless an active dialect depends on it.
    pub fn deactivate(&mut self, id: &DialectId) -> Result<(), DialectError> {
        if !self.entries.contains_key(id) {
            return Err(DialectError::UnknownDialect { id: id.clone() });
        }

        for entry in self.entries.values() {
            if entry.is_active()
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

        let entry = self
            .entries
            .get_mut(id)
            .ok_or_else(|| DialectError::UnknownDialect { id: id.clone() })?;

        entry.state = DialectState::Inactive;
        Ok(())
    }

    /// Iterates over active dialect definitions in identifier order.
    pub fn active(&self) -> impl Iterator<Item = &DialectDefinition> {
        self.entries
            .values()
            .filter(|entry| entry.is_active())
            .map(|entry| &entry.definition)
    }

    /// Resolves all active dialects into cloned canonical keyword and
    /// operator registries.
    ///
    /// Neither input registry is mutated. Any conflict or registration
    /// failure returns an error rather than a partially resolved result.
    ///
    /// Conflict checking uses ordered spelling indexes rather than
    /// repeatedly scanning every registered definition.
    ///
    /// This method does not alter an already constructed scanner. A caller
    /// must explicitly construct or reconfigure a scanner with the returned
    /// registries.
    pub fn resolve(
        &self,
        core_keywords: &KeywordRegistry,
        core_operators: &OperatorRegistry,
    ) -> Result<(KeywordRegistry, OperatorRegistry), DialectError> {
        let mut keywords = core_keywords.clone();
        let mut operators = core_operators.clone();

        let mut keyword_spellings: BTreeSet<String> = keywords
            .definitions()
            .map(|definition| definition.spelling().to_owned())
            .collect();

        let mut operator_spellings: BTreeSet<String> = operators
            .definitions()
            .map(|definition| definition.spelling().to_owned())
            .collect();

        for entry in self.entries.values().filter(|entry| entry.is_active()) {
            let definition = &entry.definition;
            let dialect_name = definition.id().as_str().to_owned();

            // Validate the complete dialect against previously accepted
            // spellings before changing either candidate registry.
            for keyword in definition.keywords() {
                let spelling = keyword.spelling();

                if keyword_spellings.contains(spelling) {
                    return Err(DialectError::KeywordConflict {
                        dialect: definition.id().clone(),
                        spelling: spelling.to_owned(),
                    });
                }

                if operator_spellings.contains(spelling) {
                    return Err(DialectError::CrossCategoryConflict {
                        dialect: definition.id().clone(),
                        spelling: spelling.to_owned(),
                    });
                }
            }

            for operator in definition.operators() {
                let spelling = operator.spelling();

                if operator_spellings.contains(spelling) {
                    return Err(DialectError::OperatorConflict {
                        dialect: definition.id().clone(),
                        spelling: spelling.to_owned(),
                    });
                }

                if keyword_spellings.contains(spelling) {
                    return Err(DialectError::CrossCategoryConflict {
                        dialect: definition.id().clone(),
                        spelling: spelling.to_owned(),
                    });
                }
            }

            let dialect_keywords = definition.keywords().cloned().map(|keyword| {
                keyword.with_origin(KeywordOrigin::Dialect(dialect_name.clone()))
            });

            keywords.register_batch(dialect_keywords)?;

            let dialect_operators = definition.operators().cloned().map(|operator| {
                operator.with_origin(TokenOrigin::Dialect(dialect_name.clone()))
            });

            operators.register_batch(dialect_operators)?;

            keyword_spellings.extend(
                definition
                    .keywords()
                    .map(|keyword| keyword.spelling().to_owned()),
            );

            operator_spellings.extend(
                definition
                    .operators()
                    .map(|operator| operator.spelling().to_owned()),
            );
        }

        Ok((keywords, operators))
    }
}

/// Dialect registration, validation, activation, and resolution errors.
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
    ConflictingDependencyVersions {
        dialect: DialectId,
        dependency: DialectId,
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                f.write_str("dialect identifier must not be empty")
            }
            Self::InvalidIdentifier { value, reason } => {
                write!(f, "invalid dialect identifier `{value}`: {reason}")
            }
            Self::EmptyVersion => {
                f.write_str("dialect version must not be empty")
            }
            Self::InvalidVersion { value } => {
                write!(f, "invalid dialect version `{value}`")
            }
            Self::EmptyDescription { dialect } => {
                write!(f, "dialect `{dialect}` has an empty description")
            }
            Self::DuplicateDialect { id } => {
                write!(f, "dialect `{id}` is already registered")
            }
            Self::UnknownDialect { id } => {
                write!(f, "dialect `{id}` is not registered")
            }
            Self::DuplicateDialectKeyword { dialect, spelling } => {
                write!(f, "dialect `{dialect}` repeats keyword `{spelling}`")
            }
            Self::DuplicateDialectOperator { dialect, spelling } => {
                write!(f, "dialect `{dialect}` repeats operator `{spelling}`")
            }
            Self::KeywordConflict { dialect, spelling } => {
                write!(f, "dialect `{dialect}` conflicts with keyword `{spelling}`")
            }
            Self::OperatorConflict { dialect, spelling } => {
                write!(f, "dialect `{dialect}` conflicts with operator `{spelling}`")
            }
            Self::CrossCategoryConflict { dialect, spelling } => {
                write!(
                    f,
                    "dialect `{dialect}` assigns `{spelling}` to conflicting lexical categories"
                )
            }
            Self::ConflictingDependencyVersions {
                dialect,
                dependency,
            } => {
                write!(
                    f,
                    "dialect `{dialect}` declares conflicting version requirements for `{dependency}`"
                )
            }
            Self::MissingDependency {
                dialect,
                dependency,
            } => {
                write!(
                    f,
                    "dialect `{dialect}` requires unregistered dialect `{dependency}`"
                )
            }
            Self::InactiveDependency {
                dialect,
                dependency,
            } => {
                write!(
                    f,
                    "dialect `{dialect}` requires inactive dialect `{dependency}`"
                )
            }
            Self::DependencyVersionMismatch {
                dialect,
                dependency,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "dialect `{dialect}` requires `{dependency}` version \
                     `{expected}`, but `{actual}` is registered"
                )
            }
            Self::RequiredByActiveDialect { id, dependent } => {
                write!(
                    f,
                    "cannot deactivate `{id}` while active dialect `{dependent}` depends on it"
                )
            }
            Self::KeywordRegistry(message) => {
                write!(f, "keyword registry rejected dialect: {message}")
            }
            Self::OperatorRegistry(message) => {
                write!(f, "operator registry rejected dialect: {message}")
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
        DialectId::new(value).expect("test identifier should be valid")
    }

    fn version(value: &str) -> DialectVersion {
        DialectVersion::new(value).expect("test version should be valid")
    }

    fn definition(name: &str) -> DialectDefinition {
        DialectDefinition::new(id(name), version("1.0.0"))
            .with_stability(DialectStability::Stable)
    }

    #[test]
    fn identifiers_reject_empty_whitespace_and_controls() {
        assert!(matches!(
            DialectId::new(""),
            Err(DialectError::EmptyIdentifier)
        ));

        assert!(matches!(
            DialectId::new("   "),
            Err(DialectError::EmptyIdentifier)
        ));

        assert!(matches!(
            DialectId::new(" invalid "),
            Err(DialectError::InvalidIdentifier { .. })
        ));

        assert!(matches!(
            DialectId::new("bad\nid"),
            Err(DialectError::InvalidIdentifier { .. })
        ));

        assert!(DialectId::new("vendor.quantum").is_ok());
    }

    #[test]
    fn versions_reject_empty_whitespace_and_controls() {
        assert!(matches!(
            DialectVersion::new(""),
            Err(DialectError::EmptyVersion)
        ));

        assert!(DialectVersion::new("1.0.0").is_ok());

        assert!(matches!(
            DialectVersion::new(" 1.0.0"),
            Err(DialectError::InvalidVersion { .. })
        ));
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut registry = DialectRegistry::new();
        registry.register(definition("example")).unwrap();

        assert!(matches!(
            registry.register(definition("example")),
            Err(DialectError::DuplicateDialect { .. })
        ));

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn batch_registration_is_atomic() {
        let mut registry = DialectRegistry::new();
        registry.register(definition("existing")).unwrap();

        let result = registry.register_batch([
            definition("new"),
            definition("existing"),
        ]);

        assert!(matches!(
            result,
            Err(DialectError::DuplicateDialect { .. })
        ));

        assert_eq!(registry.len(), 1);
        assert!(registry.get(&id("new")).is_none());
    }

    #[test]
    fn dependencies_must_exist_and_be_active() {
        let dependent = definition("dependent")
            .with_dependency(DialectDependency::any_version(id("base")));

        let mut registry = DialectRegistry::new();
        registry.register(dependent).unwrap();

        assert!(matches!(
            registry.activate(&id("dependent")),
            Err(DialectError::MissingDependency { .. })
        ));

        registry.register(definition("base")).unwrap();

        assert!(matches!(
            registry.activate(&id("dependent")),
            Err(DialectError::InactiveDependency { .. })
        ));

        registry.activate(&id("base")).unwrap();
        registry.activate(&id("dependent")).unwrap();

        assert_eq!(
            registry.get(&id("dependent")).map(RegisteredState::state),
            Some(DialectState::Active)
        );
    }

    #[test]
    fn exact_dependency_version_is_enforced() {
        let dependent = definition("dependent").with_dependency(
            DialectDependency::exact(id("base"), version("2.0.0")),
        );

        let mut registry = DialectRegistry::new();
        registry.register(definition("base")).unwrap();
        registry.register(dependent).unwrap();
        registry.activate(&id("base")).unwrap();

        assert!(matches!(
            registry.activate(&id("dependent")),
            Err(DialectError::DependencyVersionMismatch { .. })
        ));
    }

    #[test]
    fn active_dependents_prevent_deactivation() {
        let mut registry = DialectRegistry::new();

        registry.register(definition("base")).unwrap();
        registry
            .register(
                definition("dependent")
                    .with_dependency(DialectDependency::any_version(id("base"))),
            )
            .unwrap();

        registry.activate(&id("base")).unwrap();
        registry.activate(&id("dependent")).unwrap();

        assert!(matches!(
            registry.deactivate(&id("base")),
            Err(DialectError::RequiredByActiveDialect { .. })
        ));
    }

    #[test]
    fn successful_deactivation_changes_state() {
        let mut registry = DialectRegistry::new();
        registry.register(definition("example")).unwrap();
        registry.activate(&id("example")).unwrap();
        registry.deactivate(&id("example")).unwrap();

        assert_eq!(
            registry.get(&id("example")).map(RegisteredState::state),
            Some(DialectState::Inactive)
        );
    }

    #[test]
    fn active_iteration_is_deterministic() {
        let mut registry = DialectRegistry::new();

        registry.register(definition("zeta")).unwrap();
        registry.register(definition("alpha")).unwrap();
        registry.activate(&id("zeta")).unwrap();
        registry.activate(&id("alpha")).unwrap();

        let names: Vec<_> = registry
            .active()
            .map(|dialect| dialect.id().as_str())
            .collect();

        assert_eq!(names, vec!["alpha", "zeta"]);
    }

    #[test]
    fn empty_registry_is_valid() {
        let registry = DialectRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
        assert_eq!(registry.active().count(), 0);
    }

    // A tiny helper avoids duplicating option unwrapping in state assertions.
    trait RegisteredState {
        fn state(self) -> DialectState;
    }

    impl RegisteredState for &super::RegisteredDialect {
        fn state(self) -> DialectState {
            super::RegisteredDialect::state(self)
        }
    }
}
