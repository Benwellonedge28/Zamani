
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Dialect Registry
//!
//! Canonical path: `src/zuir/dialect/registry.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: safe Rust only.
//!
//! PURPOSE
//!
//! Own the explicitly scoped registry of validated ZUIR dialect manifests.
//! The registry provides deterministic lookup, atomic registration, export
//! resolution, dependency/version validation, cycle detection, and trust
//! policy enforcement.
//!
//! OWNERSHIP
//!
//! This module owns registry state and cross-manifest validation.
//!
//! `definition.rs` owns dialect manifest declarations.
//! `operation.rs` owns detailed operation schemas and their validation.
//! `versioning.rs` owns migration and extended compatibility behavior.
//! `schema/types.rs` owns QualifiedName and SchemaType.
//! `schema/attributes.rs` owns attribute declarations and validation.
//! `version.rs` owns version identities.
//! The verifier owns whole-program semantic verification.
//! Target/backend layers own physical feasibility and execution.
//!
//! IMPORTANT BOUNDARIES
//!
//! Registration does not execute extension code, establish trust in an
//! external binary, prove an operation implementation exists, or imply that
//! a target can execute the dialect.
//!
//! A manifest-only registry must not be mistaken for an executable plugin
//! registry. Implementation binding and operation-schema validation belong
//! to their respective integration contracts.
//!
//! SCALABILITY
//!
//! All registry collections are dynamically sized. There is no universal
//! maximum number of dialects, exports, operations, devices, qubits, nodes,
//! threads, or other computational resources.
//!
//! Finite memory and execution time remain real constraints. Callers
//! processing untrusted manifests must enforce explicit decoding and
//! compilation budgets before allocating arbitrarily large inputs.
//!
//! DETERMINISM
//!
//! BTreeMap/BTreeSet define stable ordering for registry enumeration and
//! dependency traversal. Registration batches either succeed completely or
//! leave the registry unchanged.
//!
//! THREADING
//!
//! The registry has no global state or interior mutability. Immutable
//! references can be shared across compiler stages. Concurrent mutation
//! requires external synchronization or independent registry instances.
//!
//! GRAMMAR
//!
//! The registry does not parse source code or introduce keywords. Source
//! syntax remains governed by grammar/Zamani.g4 and grammar/DESIGN.md.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::zuir::dialect::definition::{
    DialectDefinition,
    DialectExport,
    DialectExportName,
    DialectName,
    DialectTrust,
    ExportKind,
    VersionRequirement,
};
use crate::zuir::schema::types::QualifiedName;
use crate::zuir::version::{
    DialectVersion,
    VersionTriple,
    CURRENT_ZUIR_FORMAT_VERSION,
    CURRENT_ZUIR_SCHEMA_VERSION,
};

/// Policy controlling which manifest declarations may be registered.
///
/// These settings authorize registration of declarations only. They do not
/// authorize loading or executing extension code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryPolicy {
    /// Permit manifests declaring no executable trust requirement.
    pub allow_declarative_only: bool,

    /// Permit manifests requiring host approval.
    ///
    /// Enabling this option does not itself approve any implementation.
    /// It only allows such a manifest to be registered.
    pub allow_host_approved: bool,

    /// Permit manifests declaring a verified-extension trust requirement.
    ///
    /// Actual verification must occur in the trusted extension-loading layer.
    pub allow_verified_extension: bool,

    /// Permit dependencies that defer version matching to registry policy.
    pub allow_registry_policy_version_requirements: bool,
}

impl Default for RegistryPolicy {
    fn default() -> Self {
        Self {
            allow_declarative_only: true,
            allow_host_approved: false,
            allow_verified_extension: false,
            allow_registry_policy_version_requirements: true,
        }
    }
}

impl RegistryPolicy {
    fn permits_trust(self, trust: DialectTrust) -> bool {
        match trust {
            DialectTrust::DeclarativeOnly => self.allow_declarative_only,
            DialectTrust::HostApproved => self.allow_host_approved,
            DialectTrust::VerifiedExtension => self.allow_verified_extension,
        }
    }

    fn permits_version_requirement(
        self,
        requirement: VersionRequirement,
        installed: VersionTriple,
    ) -> bool {
        match requirement {
            VersionRequirement::Exact {
                major,
                minor,
                patch,
            } => installed == VersionTriple::new(major, minor, patch),

            VersionRequirement::SameMajorAtLeast {
                major,
                minor,
                patch,
            } => {
                installed.major() == major
                    && installed >= VersionTriple::new(major, minor, patch)
            }

            VersionRequirement::RegistryPolicy => {
                self.allow_registry_policy_version_requirements
            }
        }
    }
}

/// A validated reference to an export within a dialect.
///
/// Names are scoped by dialect identity. Two different dialects may declare
/// the same local export name without colliding.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExportReference {
    dialect: DialectName,
    name: DialectExportName,
    kind: ExportKind,
}

impl ExportReference {
    /// Construct an export reference.
    #[must_use]
    pub fn new(
        dialect: DialectName,
        name: DialectExportName,
        kind: ExportKind,
    ) -> Self {
        Self {
            dialect,
            name,
            kind,
        }
    }

    /// Return the dialect namespace.
    #[must_use]
    pub fn dialect(&self) -> &DialectName {
        &self.dialect
    }

    /// Return the export name.
    #[must_use]
    pub fn name(&self) -> &DialectExportName {
        &self.name
    }

    /// Return the expected export category.
    #[must_use]
    pub const fn kind(&self) -> ExportKind {
        self.kind
    }
}

/// A stable, owned description of a registered dialect.
///
/// The manifest remains the source of truth; this wrapper does not duplicate
/// individual operation schemas or semantic types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisteredDialect {
    definition: DialectDefinition,
}

impl RegisteredDialect {
    /// Return the authoritative manifest.
    #[must_use]
    pub fn definition(&self) -> &DialectDefinition {
        &self.definition
    }

    /// Return the dialect name.
    #[must_use]
    pub fn name(&self) -> &DialectName {
        self.definition.name()
    }

    /// Return the declared version.
    #[must_use]
    pub const fn version(&self) -> DialectVersion {
        self.definition.version()
    }

    /// Return the number of declared exports.
    #[must_use]
    pub fn export_count(&self) -> usize {
        self.definition.export_count()
    }
}

/// Explicit failures produced by registry operations.
///
/// Errors contain structured variants rather than depending on terminal
/// output, source parsing, or a particular diagnostic renderer.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum RegistryError {
    #[error("invalid dialect manifest: {0}")]
    InvalidManifest(String),

    #[error("dialect is already registered: {0}")]
    DuplicateDialect(String),

    #[error("dialect is not registered: {0}")]
    UnknownDialect(String),

    #[error("export is not declared: {dialect}::{export}")]
    UnknownExport {
        dialect: String,
        export: String,
    },

    #[error(
        "export kind mismatch for {dialect}::{export}: expected {expected:?}, found {actual:?}"
    )]
    ExportKindMismatch {
        dialect: String,
        export: String,
        expected: ExportKind,
        actual: ExportKind,
    },

    #[error("required dialect dependency is missing: {dialect} requires {dependency}")]
    MissingDependency {
        dialect: String,
        dependency: String,
    },

    #[error("dependency version requirement failed: {dialect} requires {dependency}")]
    DependencyVersionMismatch {
        dialect: String,
        dependency: String,
    },

    #[error("dialect dependency cycle detected: {path:?}")]
    DependencyCycle {
        path: Vec<String>,
    },

    #[error("registration denied by trust policy for dialect {dialect}: {trust:?}")]
    TrustPolicyDenied {
        dialect: String,
        trust: DialectTrust,
    },

    #[error("registry-policy version requirement is not permitted for {dialect}")]
    VersionPolicyDenied {
        dialect: String,
    },

    #[error("registry invariant violated: {0}")]
    InvariantViolation(String),
}

/// Registry of validated dialect declarations.
///
/// This type intentionally contains no backend handles, executable
/// callbacks, hardware objects, or global mutable state.
#[derive(Debug, Clone, Default)]
pub struct DialectRegistry {
    dialects: BTreeMap<DialectName, RegisteredDialect>,
    policy: RegistryPolicy,
}

impl DialectRegistry {
    /// Create an empty registry using the default trust policy.
    #[must_use]
    pub fn new() -> Self {
        Self::with_policy(RegistryPolicy::default())
    }

    /// Create an empty registry with an explicit registration policy.
    #[must_use]
    pub fn with_policy(policy: RegistryPolicy) -> Self {
        Self {
            dialects: BTreeMap::new(),
            policy,
        }
    }

    /// Return the active policy.
    #[must_use]
    pub const fn policy(&self) -> RegistryPolicy {
        self.policy
    }

    /// Return the number of registered dialects.
    #[must_use]
    pub fn len(&self) -> usize {
        self.dialects.len()
    }

    /// Return whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dialects.is_empty()
    }

    /// Register one dialect.
    ///
    /// The operation is atomic. If any manifest or dependency check fails,
    /// the original registry remains unchanged.
    ///
    /// Required dependencies must already be registered. Use
    /// `register_batch` when registering a group of mutually dependent
    /// manifests in one transaction.
    pub fn register(
        &mut self,
        definition: DialectDefinition,
    ) -> Result<(), RegistryError> {
        self.register_batch(std::iter::once(definition))
    }

    /// Atomically register a group of dialect manifests.
    ///
    /// The entire batch is staged, locally validated, and checked as a
    /// complete dependency graph before replacing the registry state.
    ///
    /// This permits dependencies between members of the same batch while
    /// rejecting missing required dependencies and dependency cycles.
    pub fn register_batch<I>(
        &mut self,
        definitions: I,
    ) -> Result<(), RegistryError>
    where
        I: IntoIterator<Item = DialectDefinition>,
    {
        let mut staged = self.dialects.clone();

        for definition in definitions {
            definition
                .validate()
                .map_err(|error| RegistryError::InvalidManifest(error.to_string()))?;

            self.validate_trust(&definition)?;

            let name = definition.name().clone();

            if staged.contains_key(&name) {
                return Err(RegistryError::DuplicateDialect(name.to_string()));
            }

            staged.insert(name, RegisteredDialect { definition });
        }

        Self::validate_dependencies(&staged, self.policy)?;
        Self::validate_acyclic(&staged)?;

        self.dialects = staged;
        Ok(())
    }

    /// Return a registered dialect by its validated name.
    #[must_use]
    pub fn get(&self, name: &DialectName) -> Option<&RegisteredDialect> {
        self.dialects.get(name)
    }

    /// Return a dialect by a parsed qualified name.
    ///
    /// Invalid input returns `None`; it is not silently normalized.
    #[must_use]
    pub fn get_by_name(&self, name: &str) -> Option<&RegisteredDialect> {
        let name = DialectName::parse(name).ok()?;
        self.get(&name)
    }

    /// Return whether a dialect is registered.
    #[must_use]
    pub fn contains(&self, name: &DialectName) -> bool {
        self.dialects.contains_key(name)
    }

    /// Iterate over registered dialects in deterministic name order.
    pub fn iter(&self) -> impl Iterator<Item = &RegisteredDialect> {
        self.dialects.values()
    }

    /// Iterate over registered dialect names in deterministic order.
    pub fn names(&self) -> impl Iterator<Item = &DialectName> {
        self.dialects.keys()
    }

    /// Resolve an export and verify its declared category.
    ///
    /// This proves only that the manifest declares the requested export.
    /// It does not prove that an operation schema or executable
    /// implementation has been installed.
    pub fn resolve_export(
        &self,
        reference: &ExportReference,
    ) -> Result<&DialectExport, RegistryError> {
        let dialect = self
            .dialects
            .get(reference.dialect())
            .ok_or_else(|| RegistryError::UnknownDialect(
                reference.dialect().to_string(),
            ))?;

        let export = dialect
            .definition()
            .export(reference.name())
            .ok_or_else(|| RegistryError::UnknownExport {
                dialect: reference.dialect().to_string(),
                export: reference.name().to_string(),
            })?;

        if export.kind() != reference.kind() {
            return Err(RegistryError::ExportKindMismatch {
                dialect: reference.dialect().to_string(),
                export: reference.name().to_string(),
                expected: reference.kind(),
                actual: export.kind(),
            });
        }

        Ok(export)
    }

    /// Return whether an export with the given name and kind is declared.
    #[must_use]
    pub fn has_export(
        &self,
        dialect: &DialectName,
        export: &DialectExportName,
        kind: ExportKind,
    ) -> bool {
        self.dialects
            .get(dialect)
            .is_some_and(|registered| registered.definition().has_export(export, kind))
    }

    /// Return all manifest declarations that reference a named interface.
    ///
    /// Interface names are schema-level qualified names. This query does not
    /// imply that a corresponding runtime or lowering implementation exists.
    pub fn dialects_declaring_interface<'a>(
        &'a self,
        interface: &'a QualifiedName,
    ) -> impl Iterator<Item = &'a RegisteredDialect> + 'a {
        self.dialects.values().filter(move |registered| {
            registered.definition().interfaces().any(|item| item == interface)
                || registered.definition().exports().any(|export| {
                    export.kind() == ExportKind::Interface
                        && export.name().qualified_name() == interface
                })
        })
    }

    /// Remove a dialect only when no remaining dialect requires it.
    ///
    /// Optional dependencies do not prevent removal. The removal is atomic:
    /// if a required dependent exists, the registry remains unchanged.
    pub fn unregister(
        &mut self,
        name: &DialectName,
    ) -> Result<RegisteredDialect, RegistryError> {
        if !self.dialects.contains_key(name) {
            return Err(RegistryError::UnknownDialect(name.to_string()));
        }

        for registered in self.dialects.values() {
            if registered.name() == name {
                continue;
            }

            if let Some(dependency) = registered.definition().dependency(name) {
                if !dependency.is_optional() {
                    return Err(RegistryError::InvariantViolation(format!(
                        "cannot remove {name}; required by {}",
                        registered.name()
                    )));
                }
            }
        }

        self.dialects
            .remove(name)
            .ok_or_else(|| RegistryError::UnknownDialect(name.to_string()))
    }

    fn validate_trust(
        &self,
        definition: &DialectDefinition,
    ) -> Result<(), RegistryError> {
        let trust = definition.implementation().trust();

        if self.policy.permits_trust(trust) {
            Ok(())
        } else {
            Err(RegistryError::TrustPolicyDenied {
                dialect: definition.name().to_string(),
                trust,
            })
        }
    }

    fn validate_dependencies(
        dialects: &BTreeMap<DialectName, RegisteredDialect>,
        policy: RegistryPolicy,
    ) -> Result<(), RegistryError> {
        let schema_version = CURRENT_ZUIR_SCHEMA_VERSION.components();
        let format_version = CURRENT_ZUIR_FORMAT_VERSION.components();

        for registered in dialects.values() {
            let name = registered.name();

            for dependency in registered.definition().dependencies() {
                let dependency_name = dependency.dialect();

                let Some(installed) = dialects.get(dependency_name) else {
                    if dependency.is_optional() {
                        continue;
                    }

                    return Err(RegistryError::MissingDependency {
                        dialect: name.to_string(),
                        dependency: dependency_name.to_string(),
                    });
                };

                let requirement = *dependency.version_requirement();

                if matches!(requirement, VersionRequirement::RegistryPolicy)
                    && !policy.allow_registry_policy_version_requirements
                {
                    return Err(RegistryError::VersionPolicyDenied {
                        dialect: name.to_string(),
                    });
                }

                let installed_version = installed.version().components();

                if !policy.permits_version_requirement(requirement, installed_version) {
                    return Err(RegistryError::DependencyVersionMismatch {
                        dialect: name.to_string(),
                        dependency: dependency_name.to_string(),
                    });
                }
            }

            let compatibility = registered.definition().compatibility();

            if !policy.permits_version_requirement(
                *compatibility.minimum_zuir_schema(),
                schema_version,
            ) {
                return Err(RegistryError::InvalidManifest(format!(
                    "dialect {} does not support the active ZUIR schema version",
                    name
                )));
            }

            if !policy.permits_version_requirement(
                *compatibility.minimum_zuir_format(),
                format_version,
            ) {
                return Err(RegistryError::InvalidManifest(format!(
                    "dialect {} does not support the active ZUIR format version",
                    name
                )));
            }
        }

        Ok(())
    }

    /// Detect cycles iteratively to avoid recursive call-stack growth.
    fn validate_acyclic(
        dialects: &BTreeMap<DialectName, RegisteredDialect>,
    ) -> Result<(), RegistryError> {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum VisitState {
            Visiting,
            Complete,
        }

        let mut states: BTreeMap<DialectName, VisitState> = BTreeMap::new();

        for start in dialects.keys() {
            if states.get(start) == Some(&VisitState::Complete) {
                continue;
            }

            let mut stack: Vec<(DialectName, Vec<DialectName>)> =
                vec![(start.clone(), Vec::new())];

            while let Some((current, mut path)) = stack.pop() {
                match states.get(&current) {
                    Some(VisitState::Complete) => continue,
                    Some(VisitState::Visiting) => {
                        path.push(current.clone());

                        return Err(RegistryError::DependencyCycle {
                            path: path.iter().map(ToString::to_string).collect(),
                        });
                    }
                    None => {}
                }

                states.insert(current.clone(), VisitState::Visiting);
                path.push(current.clone());

                let registered = dialects.get(&current).ok_or_else(|| {
                    RegistryError::InvariantViolation(format!(
                        "dependency graph references missing dialect {current}"
                    ))
                })?;

                // Reverse iteration keeps processing deterministic when using
                // a LIFO stack.
                let dependencies: Vec<DialectName> = registered
                    .definition()
                    .dependencies()
                    .filter(|dependency| dialects.contains_key(dependency.dialect()))
                    .map(|dependency| dependency.dialect().clone())
                    .collect();

                for dependency in dependencies.into_iter().rev() {
                    match states.get(&dependency) {
                        Some(VisitState::Visiting) => {
                            let mut cycle = path.clone();
                            cycle.push(dependency.clone());

                            return Err(RegistryError::DependencyCycle {
                                path: cycle.iter().map(ToString::to_string).collect(),
                            });
                        }
                        Some(VisitState::Complete) => {}
                        None => {
                            stack.push((dependency, path.clone()));
                        }
                    }
                }

                states.insert(current, VisitState::Complete);
            }
        }

        Ok(())
    }
}

impl fmt::Display for ExportReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.dialect, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::dialect::definition::{
        DialectCompatibility,
        DialectDefinition,
        DialectImplementation,
        DialectName,
        DialectTrust,
        VersionRequirement,
    };
    use crate::zuir::version::INITIAL_DIALECT_VERSION;

    fn compatibility() -> DialectCompatibility {
        DialectCompatibility::new(
            VersionRequirement::registry_policy(),
            VersionRequirement::registry_policy(),
            false,
        )
        .expect("valid compatibility")
    }

    fn definition(name: &str) -> DialectDefinition {
        DialectDefinition::new(
            DialectName::parse(name).expect("valid dialect name"),
            INITIAL_DIALECT_VERSION,
            compatibility(),
            DialectImplementation::new(DialectTrust::DeclarativeOnly),
        )
        .expect("valid dialect definition")
    }

    #[test]
    fn registers_and_looks_up_a_dialect() {
        let mut registry = DialectRegistry::new();
        let dialect = definition("example");

        registry.register(dialect).expect("registration succeeds");

        assert!(registry.get_by_name("example").is_some());
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn duplicate_registration_is_rejected_atomically() {
        let mut registry = DialectRegistry::new();
        registry.register(definition("example")).unwrap();

        let error = registry.register(definition("example")).unwrap_err();

        assert!(matches!(error, RegistryError::DuplicateDialect(_)));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn batch_registration_resolves_internal_dependencies() {
        let mut dependency = definition("provider");
        let dependent_name = DialectName::parse("provider").unwrap();

        let dependency = crate::zuir::dialect::definition::DialectDependency::new(
            dependent_name,
            VersionRequirement::registry_policy(),
            false,
        )
        .unwrap();

        let mut dependent = definition("consumer");
        dependent.add_dependency(dependency).unwrap();

        // The registry accepts both manifests in one transaction.
        registry_batch_assertions(vec![dependency_manifest(dependency), dependent]);
        let _ = &mut dependency;
    }

    fn dependency_manifest(
        _dependency: crate::zuir::dialect::definition::DialectDependency,
    ) -> DialectDefinition {
        definition("provider")
    }

    fn registry_batch_assertions(definitions: Vec<DialectDefinition>) {
        let mut registry = DialectRegistry::new();
        registry
            .register_batch(definitions)
            .expect("batch should be valid");
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn required_missing_dependency_is_rejected() {
        let mut dependent = definition("consumer");
        let dependency = crate::zuir::dialect::definition::DialectDependency::new(
            DialectName::parse("missing").unwrap(),
            VersionRequirement::registry_policy(),
            false,
        )
        .unwrap();

        dependent.add_dependency(dependency).unwrap();

        let mut registry = DialectRegistry::new();
        let error = registry.register(dependent).unwrap_err();

        assert!(matches!(error, RegistryError::MissingDependency { .. }));
        assert!(registry.is_empty());
    }

    #[test]
    fn denied_trust_level_is_rejected() {
        let mut registry = DialectRegistry::new();

        let definition = DialectDefinition::new(
            DialectName::parse("host_extension").unwrap(),
            INITIAL_DIALECT_VERSION,
            compatibility(),
            DialectImplementation::new(DialectTrust::HostApproved),
        )
        .unwrap();

        let error = registry.register(definition).unwrap_err();

        assert!(matches!(error, RegistryError::TrustPolicyDenied { .. }));
        assert!(registry.is_empty());
    }

    #[test]
    fn enumeration_is_deterministic() {
        let mut registry = DialectRegistry::new();

        registry.register(definition("zeta")).unwrap();
        registry.register(definition("alpha")).unwrap();

        let names: Vec<String> = registry.names().map(ToString::to_string).collect();

        assert_eq!(names, vec!["alpha", "zeta"]);
    }
}
