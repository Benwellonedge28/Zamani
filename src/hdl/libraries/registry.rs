
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDL Library Registry
//!
//! Path: `src/hdl/libraries/registry.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: Initial implementation; freeze after workspace verification.
//!
//! ## Purpose
//!
//! Provide a deterministic, in-memory registry for HDL library descriptors.
//! The registry stores logical library identities and metadata and provides
//! explicit registration, lookup, removal, and batch-registration operations.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/spec/versioning.md`
//! - `grammar/spec/determinism.md`
//! - `grammar/hdl/hdl.g4`
//! - `src/hdl/ids.rs`
//! - `src/hdl/error.rs`
//! - `src/frontend/ast/node/paths/import.rs`
//! - The repository's canonical semantic model and package-management APIs.
//!
//! The grammar owns syntax. The frontend AST owns source-level import
//! structure. Semantic resolution owns the meaning of imported names.
//! Package-management and manifest components own package acquisition,
//! manifest parsing, dependency resolution, and persistent storage.
//!
//! ## Owns
//!
//! - The in-memory HDL library descriptor contract.
//! - Exact library identity matching.
//! - Registry-local identity assignment.
//! - Deterministic registry storage and iteration.
//! - Duplicate detection and transactional batch validation.
//! - Explicit library registration and removal.
//!
//! ## Does not own
//!
//! - Parsing HDL source or library manifests.
//! - Filesystem access, network access, or package downloading.
//! - Dependency fetching, dependency solving, or version-range resolution.
//! - Cryptographic verification, artifact authentication, or trust policy.
//! - General semantic name resolution or source AST construction.
//! - Target discovery, hardware allocation, or physical implementation.
//! - HDLIR, another intermediate representation, or runtime handles.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Identity and versioning
//!
//! A library is identified by the exact tuple `(namespace, name, version)`.
//! The registry does not normalize case, rewrite identifiers, infer aliases,
//! or interpret version constraints. Producers must supply canonical identity
//! strings according to the applicable language and package specifications.
//!
//! Registry IDs are local logical handles, not persistent cross-build IDs.
//! Removing an entry never makes its ID available for reuse in that registry.
//! IDs from separate registry instances are not interchangeable merely
//! because their numeric representations happen to match.
//!
//! ## Scalability
//!
//! No HDL or hardware capacity limit is imposed on the number of registered
//! libraries, dependencies, or identity-string lengths.
//!
//! Collections grow according to available resources. Registration uses
//! checked identity allocation and reports representational exhaustion.
//! It does not wrap IDs, overwrite entries, or silently drop descriptors.
//!
//! Batch registration validates the entire batch before changing the registry.
//! Ordinary allocation failure remains subject to the Rust process allocator;
//! this API does not claim that out-of-memory conditions are recoverable.
//!
//! ## Integration
//!
//! A future `src/hdl/libraries/manifest.rs` can parse and validate a manifest,
//! then construct an `HdlLibraryDescriptor` and register it here.
//!
//! Import resolution may query this registry after package acquisition and
//! semantic identity resolution have established the relevant library scope.
//! This module does not resolve import syntax or load files.
//!
//! ## Freeze criteria
//!
//! - Canonical HDL IDs and errors are publicly available.
//! - Registration and batch registration are deterministic.
//! - Duplicate identities never silently overwrite entries.
//! - Removal does not cause ID reuse.
//! - The registry does not depend on the parser, target realization, or HDLIR.
//! - Workspace formatting, compilation, tests, and Clippy pass.

use std::collections::{BTreeMap, BTreeSet};

use crate::hdl::error::{HdlError, HdlErrorKind, HdlResult};
use crate::hdl::ids::HdlLibraryId;

/// Schema version for the in-memory HDL library registry contract.
///
/// This is not the Zamani language version, package version, or HDLIR version.
pub const HDL_LIBRARY_REGISTRY_SCHEMA_VERSION: u16 = 1;

/// Stable architectural name for this module.
pub const HDL_LIBRARY_REGISTRY_MODULE_NAME: &str = "zamani.hdl.libraries.registry";

/// Validate a required identity component without changing its spelling.
///
/// This deliberately performs structural validation only. The authoritative
/// grammar or manifest validator remains responsible for language-specific
/// identifier and version syntax.
fn validate_component(
    field: &'static str,
    value: &str,
) -> HdlResult<()> {
    if value.is_empty() {
        return Err(HdlError::invalid_input(
            field,
            "the value must not be empty",
        ));
    }

    if value.trim() != value {
        return Err(HdlError::invalid_input(
            field,
            "leading or trailing whitespace is not permitted",
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(HdlError::invalid_input(
            field,
            "control characters are not permitted",
        ));
    }

    if value.chars().any(char::is_whitespace) {
        return Err(HdlError::invalid_input(
            field,
            "whitespace is not permitted in an identity component",
        ));
    }

    Ok(())
}

/// Exact, namespace-qualified library identity.
///
/// Equality is exact. The registry does not case-fold or normalize any
/// component. Version-range interpretation belongs to the package resolver.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HdlLibraryIdentity {
    namespace: String,
    name: String,
    version: String,
}

impl HdlLibraryIdentity {
    /// Construct a validated library identity.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> HdlResult<Self> {
        let namespace = namespace.into();
        let name = name.into();
        let version = version.into();

        validate_component("library.namespace", &namespace)?;
        validate_component("library.name", &name)?;
        validate_component("library.version", &version)?;

        Ok(Self {
            namespace,
            name,
            version,
        })
    }

    /// Return the exact namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the exact library name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the exact version string.
    ///
    /// This string is not parsed or ordered using semantic-version rules here.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
}

/// A dependency declaration attached to an HDL library descriptor.
///
/// `version_requirement` is intentionally opaque. A manifest parser and
/// dependency resolver must interpret it according to the package-versioning
/// contract instead of relying on ad hoc string comparisons in this registry.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HdlLibraryDependency {
    namespace: String,
    name: String,
    version_requirement: String,
}

impl HdlLibraryDependency {
    /// Construct a dependency declaration.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        version_requirement: impl Into<String>,
    ) -> HdlResult<Self> {
        let namespace = namespace.into();
        let name = name.into();
        let version_requirement = version_requirement.into();

        validate_component("dependency.namespace", &namespace)?;
        validate_component("dependency.name", &name)?;
        validate_component(
            "dependency.version_requirement",
            &version_requirement,
        )?;

        Ok(Self {
            namespace,
            name,
            version_requirement,
        })
    }

    /// Return the dependency namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the dependency name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the unparsed version requirement.
    #[must_use]
    pub fn version_requirement(&self) -> &str {
        &self.version_requirement
    }
}

/// Metadata for one logical HDL library.
///
/// This descriptor contains declarative metadata only. It does not represent
/// loaded source files, a verified package artifact, an executable module,
/// a target implementation, or a physical hardware resource.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlLibraryDescriptor {
    identity: HdlLibraryIdentity,
    dependencies: Vec<HdlLibraryDependency>,
    origin: Option<String>,
}

impl HdlLibraryDescriptor {
    /// Construct a library descriptor with no dependencies or origin.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> HdlResult<Self> {
        Ok(Self {
            identity: HdlLibraryIdentity::new(namespace, name, version)?,
            dependencies: Vec::new(),
            origin: None,
        })
    }

    /// Return the library's exact identity.
    #[must_use]
    pub fn identity(&self) -> &HdlLibraryIdentity {
        &self.identity
    }

    /// Return the declared dependencies in their supplied order.
    #[must_use]
    pub fn dependencies(&self) -> &[HdlLibraryDependency] {
        &self.dependencies
    }

    /// Return the optional descriptive origin.
    ///
    /// This value is opaque metadata. The registry never opens it or assumes
    /// that it is a filesystem path, network URL, or trusted artifact locator.
    #[must_use]
    pub fn origin(&self) -> Option<&str> {
        self.origin.as_deref()
    }

    /// Attach an opaque, non-empty origin description.
    ///
    /// The origin is informational only; setting it does not load or verify
    /// an artifact.
    pub fn with_origin(
        mut self,
        origin: impl Into<String>,
    ) -> HdlResult<Self> {
        let origin = origin.into();

        if origin.is_empty() || origin.trim() != origin {
            return Err(HdlError::invalid_input(
                "library.origin",
                "the origin must be non-empty and have no surrounding whitespace",
            ));
        }

        if origin.chars().any(char::is_control) {
            return Err(HdlError::invalid_input(
                "library.origin",
                "control characters are not permitted",
            ));
        }

        self.origin = Some(origin);
        Ok(self)
    }

    /// Replace the dependency list after validating it.
    ///
    /// Two declarations for the same namespace and name are rejected, even
    /// when their version requirements differ. Combining or resolving version
    /// constraints belongs to the dependency resolver.
    pub fn with_dependencies(
        mut self,
        dependencies: Vec<HdlLibraryDependency>,
    ) -> HdlResult<Self> {
        let mut seen = BTreeSet::new();

        for dependency in &dependencies {
            let key = (
                dependency.namespace().to_owned(),
                dependency.name().to_owned(),
            );

            if !seen.insert(key) {
                return Err(HdlError::invalid_input(
                    "library.dependencies",
                    format!(
                        "duplicate dependency declaration for `{}::{}`",
                        dependency.namespace(),
                        dependency.name(),
                    ),
                ));
            }
        }

        self.dependencies = dependencies;
        Ok(self)
    }
}

/// Deterministic, in-memory registry of HDL library descriptors.
///
/// The registry maintains two indexes:
///
/// - identity to registry-local ID;
/// - registry-local ID to descriptor.
///
/// Ordered maps make lookup and iteration behavior independent of hash seeds.
/// Iteration is ordered by registry-local ID, not by semantic-version order.
///
/// A registry is intentionally an ordinary owned value. It has no global
/// mutable state, filesystem access, network access, or implicit cache.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HdlLibraryRegistry {
    by_identity: BTreeMap<HdlLibraryIdentity, HdlLibraryId>,
    entries: BTreeMap<HdlLibraryId, HdlLibraryDescriptor>,
    next_id: Option<u64>,
}

impl HdlLibraryRegistry {
    /// Create an empty registry.
    ///
    /// The first allocated registry-local ID is one. This is an identity
    /// representation convention, not a library-count or hardware limit.
    #[must_use]
    pub fn new() -> Self {
        Self {
            by_identity: BTreeMap::new(),
            entries: BTreeMap::new(),
            next_id: Some(1),
        }
    }

    /// Return the number of registered descriptors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return whether the registry has no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Return whether the registry contains the exact identity.
    #[must_use]
    pub fn contains_identity(&self, identity: &HdlLibraryIdentity) -> bool {
        self.by_identity.contains_key(identity)
    }

    /// Look up an entry by its registry-local ID.
    #[must_use]
    pub fn get_by_id(
        &self,
        id: HdlLibraryId,
    ) -> Option<&HdlLibraryDescriptor> {
        self.entries.get(&id)
    }

    /// Look up a library using its exact namespace, name, and version.
    #[must_use]
    pub fn get(
        &self,
        namespace: &str,
        name: &str,
        version: &str,
    ) -> Option<&HdlLibraryDescriptor> {
        let identity = HdlLibraryIdentity {
            namespace: namespace.to_owned(),
            name: name.to_owned(),
            version: version.to_owned(),
        };

        self.get_by_identity(&identity)
    }

    /// Look up a library using a preconstructed identity.
    #[must_use]
    pub fn get_by_identity(
        &self,
        identity: &HdlLibraryIdentity,
    ) -> Option<&HdlLibraryDescriptor> {
        self.by_identity
            .get(identity)
            .and_then(|id| self.entries.get(id))
    }

    /// Return the registry-local ID for an exact identity.
    #[must_use]
    pub fn id_for(
        &self,
        identity: &HdlLibraryIdentity,
    ) -> Option<HdlLibraryId> {
        self.by_identity.get(identity).copied()
    }

    /// Register one descriptor.
    ///
    /// Existing entries are never overwritten. Registering an identity already
    /// present in this registry returns an error.
    pub fn register(
        &mut self,
        descriptor: HdlLibraryDescriptor,
    ) -> HdlResult<HdlLibraryId> {
        let mut ids = self.register_batch(std::iter::once(descriptor))?;

        ids.pop().ok_or_else(|| {
            HdlError::new(
                HdlErrorKind::InternalContractViolation,
                "single-item registration produced no identity",
            )
        })
    }

    /// Register multiple descriptors as one validated batch.
    ///
    /// All identities are checked against existing entries and against the
    /// other descriptors in the batch before the registry is mutated.
    ///
    /// IDs are assigned in input order. If validation or ID allocation fails,
    /// the existing registry remains unchanged.
    pub fn register_batch<I>(
        &mut self,
        descriptors: I,
    ) -> HdlResult<Vec<HdlLibraryId>>
    where
        I: IntoIterator<Item = HdlLibraryDescriptor>,
    {
        let descriptors: Vec<HdlLibraryDescriptor> =
            descriptors.into_iter().collect();

        if descriptors.is_empty() {
            return Ok(Vec::new());
        }

        let mut seen = BTreeSet::new();

        for descriptor in &descriptors {
            let identity = descriptor.identity();

            if self.by_identity.contains_key(identity) {
                return Err(HdlError::invalid_input(
                    "library.identity",
                    format!(
                        "library `{}::{}` version `{}` is already registered",
                        identity.namespace(),
                        identity.name(),
                        identity.version(),
                    ),
                ));
            }

            if !seen.insert(identity.clone()) {
                return Err(HdlError::invalid_input(
                    "library.batch",
                    format!(
                        "the batch contains duplicate identity \
                         `{}::{}` version `{}`",
                        identity.namespace(),
                        identity.name(),
                        identity.version(),
                    ),
                ));
            }
        }

        // Preflight all IDs before changing either index. `None` means the
        // finite numeric identity representation has been exhausted.
        let mut next_id = self.next_id;
        let mut prepared: Vec<(HdlLibraryId, HdlLibraryDescriptor)> =
            Vec::with_capacity(descriptors.len());
        let mut assigned_ids = Vec::with_capacity(descriptors.len());

        for descriptor in descriptors {
            let raw_id = next_id.ok_or_else(|| {
                HdlError::new(
                    HdlErrorKind::ResourceBudgetExhausted,
                    "HDL library identity representation is exhausted",
                )
            })?;

            let id = HdlLibraryId::new(raw_id).ok_or_else(|| {
                HdlError::new(
                    HdlErrorKind::InternalContractViolation,
                    "the registry attempted to construct an invalid library ID",
                )
            })?;

            // The maximum representable ID may be used once. Exhaustion is
            // recorded explicitly rather than wrapping or reusing an ID.
            next_id = raw_id.checked_add(1);

            assigned_ids.push(id);
            prepared.push((id, descriptor));
        }

        // Commit only after all validation and ID allocation have succeeded.
        for (id, descriptor) in prepared {
            let identity = descriptor.identity().clone();

            self.by_identity.insert(identity, id);
            self.entries.insert(id, descriptor);
        }

        self.next_id = next_id;

        Ok(assigned_ids)
    }

    /// Remove an entry by registry-local ID.
    ///
    /// The removed ID is never reused by this registry.
    #[must_use]
    pub fn remove(
        &mut self,
        id: HdlLibraryId,
    ) -> Option<HdlLibraryDescriptor> {
        let descriptor = self.entries.remove(&id)?;
        self.by_identity.remove(descriptor.identity());
        Some(descriptor)
    }

    /// Remove the entry with the exact supplied identity.
    ///
    /// Returns `None` when the identity is not registered.
    #[must_use]
    pub fn remove_by_identity(
        &mut self,
        identity: &HdlLibraryIdentity,
    ) -> Option<HdlLibraryDescriptor> {
        let id = self.by_identity.get(identity).copied()?;
        self.remove(id)
    }

    /// Remove all entries without resetting the identity allocator.
    ///
    /// Keeping the allocator monotonic prevents removed registry-local IDs
    /// from being silently assigned to unrelated future entries.
    pub fn clear(&mut self) {
        self.by_identity.clear();
        self.entries.clear();
    }

    /// Iterate over registered entries in ascending registry-local ID order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (HdlLibraryId, &HdlLibraryDescriptor)> {
        self.entries
            .iter()
            .map(|(id, descriptor)| (*id, descriptor))
    }

    /// Find all registered versions of a library name in a namespace.
    ///
    /// Results follow registry-local ID order. This function deliberately
    /// does not select the newest version or evaluate version requirements.
    pub fn find_versions<'a>(
        &'a self,
        namespace: &'a str,
        name: &'a str,
    ) -> impl Iterator<Item = (HdlLibraryId, &'a HdlLibraryDescriptor)> + 'a
    {
        self.entries.iter().filter_map(move |(id, descriptor)| {
            let identity = descriptor.identity();

            (identity.namespace() == namespace && identity.name() == name)
                .then_some((*id, descriptor))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn descriptor(
        namespace: &str,
        name: &str,
        version: &str,
    ) -> HdlLibraryDescriptor {
        HdlLibraryDescriptor::new(namespace, name, version)
            .expect("test descriptor should be valid")
    }

    #[test]
    fn registration_and_exact_lookup_work() {
        let mut registry = HdlLibraryRegistry::new();
        let item = descriptor("zamani.hdl", "logic", "1.0.0");

        let id = registry.register(item).expect("registration should succeed");

        assert_eq!(registry.len(), 1);
        assert_eq!(
            registry.get("zamani.hdl", "logic", "1.0.0")
                .expect("entry should exist")
                .identity()
                .name(),
            "logic",
        );
        assert!(registry.get_by_id(id).is_some());
    }

    #[test]
    fn duplicate_exact_identity_is_rejected_without_overwrite() {
        let mut registry = HdlLibraryRegistry::new();

        registry
            .register(descriptor("zamani.hdl", "logic", "1.0.0"))
            .expect("first registration should succeed");

        assert!(
            registry
                .register(descriptor("zamani.hdl", "logic", "1.0.0"))
                .is_err()
        );

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn different_versions_are_distinct_exact_identities() {
        let mut registry = HdlLibraryRegistry::new();

        registry
            .register_batch([
                descriptor("zamani.hdl", "logic", "1.0.0"),
                descriptor("zamani.hdl", "logic", "2.0.0"),
            ])
            .expect("different versions should be accepted");

        assert_eq!(
            registry
                .find_versions("zamani.hdl", "logic")
                .count(),
            2,
        );
    }

    #[test]
    fn batch_duplicate_failure_leaves_registry_unchanged() {
        let mut registry = HdlLibraryRegistry::new();

        registry
            .register(descriptor("zamani.hdl", "existing", "1.0.0"))
            .expect("initial registration should succeed");

        let before = registry.clone();

        let result = registry.register_batch([
            descriptor("zamani.hdl", "new", "1.0.0"),
            descriptor("zamani.hdl", "new", "1.0.0"),
        ]);

        assert!(result.is_err());
        assert_eq!(registry, before);
    }

    #[test]
    fn batch_conflict_with_existing_entry_is_transactional() {
        let mut registry = HdlLibraryRegistry::new();

        registry
            .register(descriptor("zamani.hdl", "existing", "1.0.0"))
            .expect("initial registration should succeed");

        let before = registry.clone();

        let result = registry.register_batch([
            descriptor("zamani.hdl", "new", "1.0.0"),
            descriptor("zamani.hdl", "existing", "1.0.0"),
        ]);

        assert!(result.is_err());
        assert_eq!(registry, before);
    }

    #[test]
    fn removal_does_not_reuse_registry_local_ids() {
        let mut registry = HdlLibraryRegistry::new();

        let first = registry
            .register(descriptor("zamani.hdl", "first", "1.0.0"))
            .expect("first registration should succeed");

        registry.remove(first).expect("entry should be removable");

        let second = registry
            .register(descriptor("zamani.hdl", "second", "1.0.0"))
            .expect("second registration should succeed");

        assert_ne!(first, second);
        assert!(registry.get_by_id(first).is_none());
        assert!(registry.get_by_id(second).is_some());
    }

    #[test]
    fn clear_does_not_reset_identity_allocation() {
        let mut registry = HdlLibraryRegistry::new();

        let first = registry
            .register(descriptor("zamani.hdl", "first", "1.0.0"))
            .expect("first registration should succeed");

        registry.clear();

        let second = registry
            .register(descriptor("zamani.hdl", "second", "1.0.0"))
            .expect("second registration should succeed");

        assert_ne!(first, second);
        assert!(registry.is_empty() == false || registry.len() == 1);
    }

    #[test]
    fn invalid_identity_components_are_rejected() {
        assert!(HdlLibraryIdentity::new("", "logic", "1.0.0").is_err());
        assert!(HdlLibraryIdentity::new("zamani.hdl", "", "1.0.0").is_err());
        assert!(HdlLibraryIdentity::new("zamani.hdl", "logic", " ").is_err());
        assert!(
            HdlLibraryIdentity::new("zamani.hdl", "bad name", "1.0.0")
                .is_err()
        );
    }

    #[test]
    fn duplicate_dependency_names_are_rejected() {
        let dependencies = vec![
            HdlLibraryDependency::new("zamani.hdl", "logic", "^1.0")
                .expect("dependency should be valid"),
            HdlLibraryDependency::new("zamani.hdl", "logic", "^2.0")
                .expect("dependency should be valid"),
        ];

        let result = descriptor("zamani.hdl", "top", "1.0.0")
            .with_dependencies(dependencies);

        assert!(result.is_err());
    }

    #[test]
    fn descriptors_preserve_dependency_order_and_origin() {
        let dependencies = vec![
            HdlLibraryDependency::new("zamani.hdl", "logic", "^1.0")
                .expect("dependency should be valid"),
            HdlLibraryDependency::new("zamani.hdl", "memory", "^2.0")
                .expect("dependency should be valid"),
        ];

        let item = descriptor("zamani.hdl", "top", "1.0.0")
            .with_dependencies(dependencies)
            .expect("dependencies should be valid")
            .with_origin("package:example/top")
            .expect("origin should be valid");

        assert_eq!(item.dependencies().len(), 2);
        assert_eq!(item.dependencies()[0].name(), "logic");
        assert_eq!(item.dependencies()[1].name(), "memory");
        assert_eq!(item.origin(), Some("package:example/top"));
    }

    #[test]
    fn iteration_is_deterministic_by_registry_id() {
        let mut registry = HdlLibraryRegistry::new();

        let first = registry
            .register(descriptor("zamani.hdl", "zeta", "1.0.0"))
            .expect("registration should succeed");

        let second = registry
            .register(descriptor("zamani.hdl", "alpha", "1.0.0"))
            .expect("registration should succeed");

        let ids: Vec<_> = registry.iter().map(|(id, _)| id).collect();

        assert_eq!(ids, vec![first, second]);
    }

    #[test]
    fn registering_an_empty_batch_is_a_no_op() {
        let mut registry = HdlLibraryRegistry::new();

        let ids = registry
            .register_batch(std::iter::empty())
            .expect("empty batch should succeed");

        assert!(ids.is_empty());
        assert!(registry.is_empty());
    }
}
