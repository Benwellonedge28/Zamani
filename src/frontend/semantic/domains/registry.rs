#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Domain Registry
//!
//! File: `src/frontend/semantic/domains/registry.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide validated registration, deterministic lookup, versioned metadata,
//! and immutable snapshots for source-level computational domains.
//!
//! ## AUTHORITY
//!
//! - `grammar/specification/domains.md`
//! - `grammar/specification/extensibility.md`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `src/frontend/ast/node/domains/domain.rs`
//! - `src/frontend/semantic/context.rs`
//! - `src/frontend/semantic/config.rs`
//!
//! ## OWNS
//!
//! - Domain definition metadata used during semantic resolution.
//! - Registration-time identity and metadata validation.
//! - Duplicate and ambiguous qualified-identity detection.
//! - Immutable, deterministically ordered registry snapshots.
//! - Extension-policy-aware domain lookup.
//! - The `SemanticExtensionCatalog` implementation for this registry.
//!
//! ## DOES_NOT_OWN
//!
//! - AST domain identity or AST construction.
//! - Domain-specific type rules, expression rules, or validation algorithms.
//! - Capability evidence from a physical target.
//! - Resource allocation, placement, scheduling, routing, or execution.
//! - Parser grammar, source syntax, or a competing semantic model.
//! - Domain-specific IRs or backend implementations.
//!
//! ## INTEGRATION CONTRACT
//!
//! `Domain` from the canonical AST is the sole domain identity.
//! `ExtensionPolicy` and `SemanticExtensionCatalog` are consumed from the
//! existing semantic configuration and context modules.
//!
//! Domain validators resolve their domain through this registry, then apply
//! their own rules through the shared semantic pipeline. This file does not
//! dispatch into validators because the validator API belongs to the semantic
//! pipeline and must remain independent of registry storage.
//!
//! ## DETERMINISM AND SCALABILITY
//!
//! Definitions are stored in ordered maps and sets. Iteration order is stable
//! for a given snapshot. Registration has no arbitrary domain-count limit.
//! The registry does not impose machine, qubit, device, tensor, or topology
//! ceilings.
//!
//! A registry revision is supplied by its owner. The owner MUST assign a new
//! revision whenever the semantic contents of a published snapshot change.
//! Revisions are explicit identifiers, not timestamps or inferred hashes.
//!
//! ## IMMUTABILITY
//!
//! The builder is mutable during assembly. `build` consumes it and produces an
//! immutable snapshot. Updating a registry means constructing and publishing
//! a new snapshot; an existing snapshot never changes in place.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::sync::Arc;

use crate::frontend::ast::node::domains::domain::{Domain, DomainError};
use crate::frontend::semantic::config::ExtensionPolicy;
use crate::frontend::semantic::context::{
    RegistryIdentity, RegistryIdentityError, SemanticExtensionCatalog,
};

/// Semantic version of a domain's semantic contract.
///
/// This is independent of the Zamani language version, compiler version,
/// registry revision, AST schema version, and target version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DomainVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl DomainVersion {
    /// Construct a domain semantic-contract version.
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Return the major version.
    #[must_use]
    pub const fn major(self) -> u64 {
        self.major
    }

    /// Return the minor version.
    #[must_use]
    pub const fn minor(self) -> u64 {
        self.minor
    }

    /// Return the patch version.
    #[must_use]
    pub const fn patch(self) -> u64 {
        self.patch
    }
}

impl fmt::Display for DomainVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Registration origin of a domain.
///
/// Core domains are part of the language implementation's trusted domain
/// definitions. Extension domains are separately registered contributions.
/// The registry does not infer origin from a domain's name or namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DomainOrigin {
    /// A domain registered by the language implementation.
    Core,

    /// A domain registered through the extension mechanism.
    Extension,
}

/// Stability classification for an extension domain.
///
/// Stability is metadata used by semantic policy. It is not evidence that a
/// domain's implementation is complete unless the repository's release and
/// conformance processes establish that status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DomainStability {
    /// The extension is eligible under `ExtensionPolicy::StableOnly`.
    Stable,

    /// The extension requires `ExtensionPolicy::AllowExperimental`.
    Experimental,
}

/// Validated metadata for one registered computational domain.
///
/// The canonical [`Domain`] is retained directly. No second domain-identity
/// type is introduced here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainDefinition {
    id: Domain,
    version: DomainVersion,
    origin: DomainOrigin,
    stability: DomainStability,
    description: Arc<str>,
}

impl DomainDefinition {
    /// Return the canonical AST domain identity.
    #[must_use]
    pub fn id(&self) -> &Domain {
        &self.id
    }

    /// Return the domain's semantic-contract version.
    #[must_use]
    pub const fn version(&self) -> DomainVersion {
        self.version
    }

    /// Return whether this definition is core or an extension.
    #[must_use]
    pub const fn origin(&self) -> DomainOrigin {
        self.origin
    }

    /// Return the stability classification.
    #[must_use]
    pub const fn stability(&self) -> DomainStability {
        self.stability
    }

    /// Return the human-readable purpose of the domain.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Return the canonical qualified domain identifier.
    ///
    /// The textual format is inherited from the AST's `Domain` implementation.
    #[must_use]
    pub fn qualified_id(&self) -> String {
        self.id.qualified_name()
    }
}

/// Errors that may occur while constructing or registering domain definitions.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DomainRegistryError {
    /// The registry namespace is invalid.
    InvalidRegistryIdentity(RegistryIdentityError),

    /// The AST rejected the domain identity.
    InvalidDomain(DomainError),

    /// The domain description is empty or whitespace-only.
    EmptyDescription,

    /// The same canonical domain identity was registered more than once.
    DuplicateDomain {
        /// The canonical qualified identity.
        qualified_id: String,
    },

    /// Two distinct AST identities have the same textual qualified identity.
    ///
    /// This is rejected because the semantic extension catalog API accepts
    /// textual IDs and therefore requires an unambiguous textual index.
    AmbiguousQualifiedIdentity {
        /// The colliding textual identity.
        qualified_id: String,
    },
}

impl fmt::Display for DomainRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRegistryIdentity(error) => {
                write!(f, "invalid domain-registry identity: {error}")
            }
            Self::InvalidDomain(error) => {
                write!(f, "invalid domain identity: {error}")
            }
            Self::EmptyDescription => {
                f.write_str("domain description must not be empty")
            }
            Self::DuplicateDomain { qualified_id } => {
                write!(f, "domain `{qualified_id}` is already registered")
            }
            Self::AmbiguousQualifiedIdentity { qualified_id } => write!(
                f,
                "distinct domain identities produce the same qualified ID \
                 `{qualified_id}`"
            ),
        }
    }
}

impl Error for DomainRegistryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidRegistryIdentity(error) => Some(error),
            Self::InvalidDomain(error) => Some(error),
            Self::EmptyDescription
            | Self::DuplicateDomain { .. }
            | Self::AmbiguousQualifiedIdentity { .. } => None,
        }
    }
}

/// Errors returned when resolving a domain against a registry snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DomainLookupError {
    /// No definition exists for the requested identity.
    UnknownDomain {
        /// The requested canonical qualified identity.
        qualified_id: String,
    },

    /// An extension domain was requested while extensions are disabled.
    ExtensionDisabled {
        /// The requested canonical qualified identity.
        qualified_id: String,
    },

    /// An experimental extension was requested under stable-only policy.
    ExperimentalDisallowed {
        /// The requested canonical qualified identity.
        qualified_id: String,
    },
}

impl fmt::Display for DomainLookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownDomain { qualified_id } => {
                write!(f, "unknown computational domain `{qualified_id}`")
            }
            Self::ExtensionDisabled { qualified_id } => write!(
                f,
                "domain extension `{qualified_id}` is disabled by policy"
            ),
            Self::ExperimentalDisallowed { qualified_id } => write!(
                f,
                "experimental domain extension `{qualified_id}` is not \
                 permitted by the active policy"
            ),
        }
    }
}

impl Error for DomainLookupError {}

/// Mutable builder used to assemble one domain-registry snapshot.
///
/// A builder may be discarded after a failed registration. Each individual
/// registration is transactional: a rejected definition does not modify the
/// builder's indexes.
#[derive(Debug)]
pub struct DomainRegistryBuilder {
    identity: RegistryIdentity,
    definitions: BTreeMap<Domain, Arc<DomainDefinition>>,
    qualified_ids: BTreeMap<String, Domain>,
    extension_ids: BTreeSet<String>,
}

impl DomainRegistryBuilder {
    /// Create a builder with an explicit immutable-snapshot identity.
    ///
    /// The namespace identifies the registry family. The revision identifies
    /// this exact snapshot within that family.
    ///
    /// # Errors
    ///
    /// Returns an error if the registry namespace is empty or whitespace-only.
    pub fn new(
        namespace: impl Into<Arc<str>>,
        revision: u64,
    ) -> Result<Self, DomainRegistryError> {
        let identity = RegistryIdentity::new(namespace, revision)
            .map_err(DomainRegistryError::InvalidRegistryIdentity)?;

        Ok(Self {
            identity,
            definitions: BTreeMap::new(),
            qualified_ids: BTreeMap::new(),
            extension_ids: BTreeSet::new(),
        })
    }

    /// Register a core domain definition.
    ///
    /// Core domains are available under all extension policies. The registry
    /// does not maintain a closed list of core domain names; the language
    /// implementation explicitly supplies the definitions it supports.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid identity, empty description, duplicate
    /// domain, or ambiguous qualified textual identity.
    pub fn register_core(
        &mut self,
        id: Domain,
        version: DomainVersion,
        description: impl Into<Arc<str>>,
    ) -> Result<(), DomainRegistryError> {
        self.register_definition(
            id,
            version,
            DomainOrigin::Core,
            DomainStability::Stable,
            description.into(),
        )
    }

    /// Register a domain extension.
    ///
    /// The caller explicitly declares its stability. Registration does not
    /// itself prove that the extension's semantic implementation is complete.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid identity, empty description, duplicate
    /// domain, or ambiguous qualified textual identity.
    pub fn register_extension(
        &mut self,
        id: Domain,
        version: DomainVersion,
        stability: DomainStability,
        description: impl Into<Arc<str>>,
    ) -> Result<(), DomainRegistryError> {
        self.register_definition(
            id,
            version,
            DomainOrigin::Extension,
            stability,
            description.into(),
        )
    }

    fn register_definition(
        &mut self,
        id: Domain,
        version: DomainVersion,
        origin: DomainOrigin,
        stability: DomainStability,
        description: Arc<str>,
    ) -> Result<(), DomainRegistryError> {
        id.validate().map_err(DomainRegistryError::InvalidDomain)?;

        if description.trim().is_empty() {
            return Err(DomainRegistryError::EmptyDescription);
        }

        let qualified_id = id.qualified_name();

        if let Some(existing) = self.qualified_ids.get(&qualified_id) {
            if existing == &id {
                return Err(DomainRegistryError::DuplicateDomain {
                    qualified_id,
                });
            }

            return Err(DomainRegistryError::AmbiguousQualifiedIdentity {
                qualified_id,
            });
        }

        let definition = Arc::new(DomainDefinition {
            id: id.clone(),
            version,
            origin,
            stability,
            description,
        });

        // All validation occurs before any index is modified. These insertions
        // cannot fail under ordinary operation except for process-level
        // allocation failure, which Rust's standard allocation behavior does
        // not expose as a recoverable error on this API.
        self.qualified_ids
            .insert(qualified_id.clone(), id.clone());

        if origin == DomainOrigin::Extension {
            self.extension_ids.insert(qualified_id);
        }

        self.definitions.insert(id, definition);
        Ok(())
    }

    /// Number of registered definitions, including core domains.
    #[must_use]
    pub fn definition_count(&self) -> usize {
        self.definitions.len()
    }

    /// Consume the builder and publish an immutable registry snapshot.
    ///
    /// The supplied revision is preserved exactly. The registry owner must
    /// ensure that changed semantic contents receive a different revision.
    #[must_use]
    pub fn build(self) -> DomainRegistry {
        DomainRegistry {
            identity: self.identity,
            definitions: self.definitions,
            qualified_ids: self.qualified_ids,
            extension_ids: self.extension_ids,
        }
    }
}

/// Immutable registry snapshot of registered computational domains.
///
/// Cloning a registry clones its immutable snapshot handles and indexes; it
/// does not create shared mutable registration state.
#[derive(Debug, Clone)]
pub struct DomainRegistry {
    identity: RegistryIdentity,
    definitions: BTreeMap<Domain, Arc<DomainDefinition>>,
    qualified_ids: BTreeMap<String, Domain>,
    extension_ids: BTreeSet<String>,
}

impl DomainRegistry {
    /// Create an empty immutable registry snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error if the registry namespace is invalid.
    pub fn empty(
        namespace: impl Into<Arc<str>>,
        revision: u64,
    ) -> Result<Self, DomainRegistryError> {
        Ok(DomainRegistryBuilder::new(namespace, revision)?.build())
    }

    /// Return this snapshot's stable identity.
    #[must_use]
    pub fn identity(&self) -> &RegistryIdentity {
        &self.identity
    }

    /// Return the number of registered definitions, including core domains.
    #[must_use]
    pub fn definition_count(&self) -> usize {
        self.definitions.len()
    }

    /// Return the number of registered extension definitions.
    #[must_use]
    pub fn extension_count(&self) -> usize {
        self.extension_ids.len()
    }

    /// Return whether a domain definition exists, regardless of policy.
    ///
    /// Use [`Self::resolve`] to determine whether the definition is permitted
    /// under the active extension policy.
    #[must_use]
    pub fn contains_domain(&self, id: &Domain) -> bool {
        self.definitions.contains_key(id)
    }

    /// Retrieve a registered definition without applying extension policy.
    ///
    /// This method is intended for metadata inspection and tooling. Semantic
    /// analysis should normally use [`Self::resolve`] instead.
    #[must_use]
    pub fn get(&self, id: &Domain) -> Option<&DomainDefinition> {
        self.definitions.get(id).map(Arc::as_ref)
    }

    /// Iterate over all definitions in deterministic identity order.
    pub fn definitions(&self) -> impl Iterator<Item = &DomainDefinition> + '_ {
        self.definitions.values().map(Arc::as_ref)
    }

    /// Resolve a domain under the active extension policy.
    ///
    /// Core definitions remain available under every `ExtensionPolicy`.
    /// Extension definitions are filtered according to the selected policy.
    ///
    /// This method checks domain registration and policy only. It does not
    /// establish target feasibility or validate domain-specific program rules.
    ///
    /// # Errors
    ///
    /// Returns an explicit error for unknown domains and extensions forbidden
    /// by the selected policy.
    pub fn resolve(
        &self,
        id: &Domain,
        policy: ExtensionPolicy,
    ) -> Result<&DomainDefinition, DomainLookupError> {
        let qualified_id = id.qualified_name();

        let definition = self
            .definitions
            .get(id)
            .map(Arc::as_ref)
            .ok_or_else(|| DomainLookupError::UnknownDomain {
                qualified_id: qualified_id.clone(),
            })?;

        if definition.origin == DomainOrigin::Core {
            return Ok(definition);
        }

        match policy {
            ExtensionPolicy::Disabled => {
                Err(DomainLookupError::ExtensionDisabled { qualified_id })
            }
            ExtensionPolicy::StableOnly
                if definition.stability == DomainStability::Experimental =>
            {
                Err(DomainLookupError::ExperimentalDisallowed {
                    qualified_id,
                })
            }
            ExtensionPolicy::StableOnly | ExtensionPolicy::AllowExperimental => {
                Ok(definition)
            }
        }
    }

    /// Resolve a textual qualified identifier under the active policy.
    ///
    /// The textual form is the canonical `Domain::qualified_name()` format.
    /// Registration rejects ambiguous textual identities, so this lookup
    /// cannot select arbitrarily between two different domain identities.
    ///
    /// # Errors
    ///
    /// Returns an explicit error for unknown or policy-disallowed domains.
    pub fn resolve_qualified(
        &self,
        qualified_id: &str,
        policy: ExtensionPolicy,
    ) -> Result<&DomainDefinition, DomainLookupError> {
        let id = self.qualified_ids.get(qualified_id).ok_or_else(|| {
            DomainLookupError::UnknownDomain {
                qualified_id: qualified_id.to_owned(),
            }
        })?;

        self.resolve(id, policy)
    }
}

impl SemanticExtensionCatalog for DomainRegistry {
    fn identity(&self) -> &RegistryIdentity {
        &self.identity
    }

    fn extension_count(&self) -> usize {
        self.extension_ids.len()
    }

    /// Check extension identifiers only; core definitions are not extensions.
    fn contains(&self, qualified_id: &str) -> bool {
        self.extension_ids.contains(qualified_id)
    }

    /// Registry entries are validated when registered. Extension policy is
    /// applied at resolution time, allowing one immutable snapshot to contain
    /// both core and experimental definitions without making core domains
    /// unusable under a restrictive policy.
    fn validate_policy(&self, _policy: ExtensionPolicy) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn domain(namespace: &str, name: &str) -> Domain {
        Domain::new(namespace, name).expect("test domain identity must be valid")
    }

    fn registry_with_domains() -> DomainRegistry {
        let mut builder =
            DomainRegistryBuilder::new("zamani.semantic.domains", 1)
                .expect("registry identity must be valid");

        builder
            .register_core(
                domain("zamani", "classical"),
                DomainVersion::new(1, 0, 0),
                "Classical computation",
            )
            .expect("core domain registration must succeed");

        builder
            .register_core(
                domain("zamani", "quantum"),
                DomainVersion::new(1, 0, 0),
                "Quantum computation",
            )
            .expect("core domain registration must succeed");

        builder
            .register_extension(
                domain("example", "experimental"),
                DomainVersion::new(0, 1, 0),
                DomainStability::Experimental,
                "Experimental extension domain",
            )
            .expect("extension registration must succeed");

        builder.build()
    }

    #[test]
    fn resolves_core_domains_under_every_policy() {
        let registry = registry_with_domains();
        let id = domain("zamani", "quantum");

        for policy in [
            ExtensionPolicy::Disabled,
            ExtensionPolicy::StableOnly,
            ExtensionPolicy::AllowExperimental,
        ] {
            let definition = registry
                .resolve(&id, policy)
                .expect("core domains must be policy-independent");

            assert_eq!(definition.id(), &id);
        }
    }

    #[test]
    fn stable_only_rejects_experimental_domains() {
        let registry = registry_with_domains();

        let result = registry.resolve(
            &domain("example", "experimental"),
            ExtensionPolicy::StableOnly,
        );

        assert!(matches!(
            result,
            Err(DomainLookupError::ExperimentalDisallowed { .. })
        ));
    }

    #[test]
    fn allow_experimental_resolves_experimental_domains() {
        let registry = registry_with_domains();

        let definition = registry
            .resolve(
                &domain("example", "experimental"),
                ExtensionPolicy::AllowExperimental,
            )
            .expect("experimental domain must be allowed");

        assert_eq!(definition.stability(), DomainStability::Experimental);
    }

    #[test]
    fn disabled_policy_rejects_extensions() {
        let registry = registry_with_domains();

        let result = registry.resolve(
            &domain("example", "experimental"),
            ExtensionPolicy::Disabled,
        );

        assert!(matches!(
            result,
            Err(DomainLookupError::ExtensionDisabled { .. })
        ));
    }

    #[test]
    fn unknown_domain_is_not_silently_accepted() {
        let registry = registry_with_domains();

        let result = registry.resolve(
            &domain("future", "not_registered"),
            ExtensionPolicy::AllowExperimental,
        );

        assert!(matches!(
            result,
            Err(DomainLookupError::UnknownDomain { .. })
        ));
    }

    #[test]
    fn duplicate_registration_is_rejected_without_mutation() {
        let mut builder =
            DomainRegistryBuilder::new("zamani.semantic.domains", 1)
                .expect("registry identity must be valid");
        let id = domain("zamani", "classical");

        builder
            .register_core(
                id.clone(),
                DomainVersion::new(1, 0, 0),
                "Classical computation",
            )
            .expect("first registration must succeed");

        let result = builder.register_core(
            id,
            DomainVersion::new(1, 0, 1),
            "Another definition",
        );

        assert!(matches!(
            result,
            Err(DomainRegistryError::DuplicateDomain { .. })
        ));
        assert_eq!(builder.definition_count(), 1);
    }

    #[test]
    fn empty_description_is_rejected_without_mutation() {
        let mut builder =
            DomainRegistryBuilder::new("zamani.semantic.domains", 1)
                .expect("registry identity must be valid");

        let result = builder.register_core(
            domain("zamani", "classical"),
            DomainVersion::new(1, 0, 0),
            "  ",
        );

        assert_eq!(result, Err(DomainRegistryError::EmptyDescription));
        assert_eq!(builder.definition_count(), 0);
    }

    #[test]
    fn definitions_iterate_in_deterministic_identity_order() {
        let registry = registry_with_domains();

        let actual: Vec<String> = registry
            .definitions()
            .map(DomainDefinition::qualified_id)
            .collect();

        let mut expected = actual.clone();
        expected.sort();

        assert_eq!(actual, expected);
    }

    #[test]
    fn extension_catalog_excludes_core_domains() {
        let registry = registry_with_domains();

        assert_eq!(registry.definition_count(), 3);
        assert_eq!(registry.extension_count(), 1);

        assert!(!SemanticExtensionCatalog::contains(
            &registry,
            "zamani.classical"
        ));
        assert!(SemanticExtensionCatalog::contains(
            &registry,
            "example.experimental"
        ));
    }

    #[test]
    fn qualified_lookup_uses_the_same_policy_checks() {
        let registry = registry_with_domains();

        let result = registry.resolve_qualified(
            "example.experimental",
            ExtensionPolicy::StableOnly,
        );

        assert!(matches!(
            result,
            Err(DomainLookupError::ExperimentalDisallowed { .. })
        ));
    }

    #[test]
    fn snapshots_keep_the_explicit_registry_identity() {
        let registry = registry_with_domains();

        assert_eq!(
            registry.identity().namespace(),
            "zamani.semantic.domains"
        );
        assert_eq!(registry.identity().revision(), 1);
    }

    #[test]
    fn catalog_policy_validation_does_not_disable_core_domains() {
        let registry = registry_with_domains();

        assert!(
            SemanticExtensionCatalog::validate_policy(
                &registry,
                ExtensionPolicy::Disabled,
            )
            .is_ok()
        );

        assert!(registry
            .resolve(
                &domain("zamani", "classical"),
                ExtensionPolicy::Disabled,
            )
            .is_ok());
    }
}