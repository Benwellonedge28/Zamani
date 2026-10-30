
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI Operation Registry
//!
//! Path: `src/ai/registry.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide an explicit, extensible, deterministic registry of AI operation
//! declarations and their versioned metadata.
//!
//! The registry allows semantic and compiler integration layers to answer:
//!
//! - Is this exact AI operation registered?
//! - Which semantic operation category does it declare?
//! - Which operation-set version does it belong to?
//! - Which schema revision describes its contract?
//! - Which registered versions exist for an operation?
//!
//! This registry records operation declarations. It does not implement,
//! execute, load, authorize, or establish the availability of operations.
//!
//! ## Canonical ownership
//!
//! This module reuses:
//!
//! - `frontend::semantic::domains::ai::QualifiedName` for AI operation names;
//! - `frontend::semantic::domains::ai::AiOperationKind` for AI operation
//!   classification;
//! - `ai::version::AiOperationSetVersion` for operation-set versioning;
//! - `ai::error::AiResult` only at higher AI integration boundaries.
//!
//! It does not introduce another operation identity, semantic type system,
//! capability model, resource model, AST, or intermediate representation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - AI operation declaration metadata;
//! - exact, versioned operation registration;
//! - duplicate detection;
//! - deterministic operation lookup and enumeration;
//! - atomic batch registration;
//! - explicit removal of registrations;
//! - immutable operation declaration snapshots;
//! - registry-specific validation errors.
//!
//! ## Does not own
//!
//! - Source syntax or annotation dispatch;
//! - the canonical frontend AST;
//! - semantic name resolution;
//! - AI operation execution semantics;
//! - canonical type or effect checking;
//! - capability requirements or capability evidence;
//! - resource requirements or feasibility;
//! - authorization or extension trust verification;
//! - executable plugin loading;
//! - model execution;
//! - tensor storage;
//! - ZUIR dialect registration;
//! - compiler optimization or lowering;
//! - hardware discovery, allocation, placement or scheduling;
//! - distributed transport or runtime execution.
//!
//! ## Important distinction: three registries
//!
//! `frontend::parser::domains::ai::AiSyntaxRegistry`
//!     Recognizes source-level AI construct names.
//!
//! `ai::registry::AiOperationRegistry`
//!     Resolves declared AI computational operation metadata.
//!
//! `zuir::dialect::registry::DialectRegistry`
//!     Registers universal IR dialect manifests and their exports.
//!
//! These registries have different ownership and must not be merged.
//!
//! ## Integration
//!
//! Frontend semantic integration:
//!     `frontend::semantic::domains::ai::AiSemanticContext`
//!
//!     The semantic adapter should resolve an `AiOperation` against this
//!     registry, then independently validate types, effects, capabilities,
//!     resources and contracts through the canonical semantic services.
//!
//! AI operation versioning:
//!     `ai::version::AiOperationSetVersion`
//!
//! AI operation classification:
//!     `frontend::semantic::domains::ai::AiOperationKind`
//!
//! AI requirements:
//!     `ai::requirements::AiRequirements`
//!
//! AI capability descriptions:
//!     `ai::capabilities`
//!
//! Universal IR dialect declarations:
//!     `zuir::dialect::registry`
//!
//! AI ZUIR operation schemas:
//!     `zuir::dialects::ai`
//!
//! Compiler, hybrid and distributed execution consume validated operation
//! declarations through their respective integration boundaries.
//!
//! The registry does not automatically register an operation merely because
//! its grammar construct or capability name exists.
//!
//! ## Versioning policy
//!
//! An operation registration is identified by:
//!
//!     (qualified operation name, operation-set version)
//!
//! Multiple versions of one operation may coexist.
//!
//! Lookup is exact. The registry never silently substitutes another version,
//! selects the newest version, or assumes semantic compatibility.
//!
//! Compatibility decisions belong to the AI compatibility subsystem and
//! applicable semantic/IR compatibility services.
//!
//! ## Atomicity and snapshots
//!
//! Registration validates all entries before publishing a batch.
//!
//! A failed batch leaves the previous registry unchanged.
//!
//! Returned `Arc` values are immutable declaration snapshots. Removing a
//! registration does not invalidate snapshots already held by consumers.
//!
//! The registry itself has no global mutable state or hidden synchronization.
//! Applications that share it across threads must provide synchronization at
//! their own ownership boundary.
//!
//! ## POCO-REAF and scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - registered AI operations;
//! - operation versions;
//! - operation-name length;
//! - extension namespaces;
//! - operation declarations;
//! - model count;
//! - model parameters;
//! - tensor rank;
//! - tensor dimensions;
//! - agents;
//! - workers;
//! - accelerators;
//! - distributed nodes;
//! - computational workload.
//!
//! Collections grow according to available memory and any explicit,
//! externally supplied operational policy.
//!
//! This module does not promise literal infinite memory or computation.
//! It introduces no language-level hardware capacity ceiling.
//!
//! ## Freeze criteria
//!
//! - Exact duplicate registrations are rejected.
//! - Distinct operation versions coexist.
//! - Unknown operations are reported explicitly.
//! - Lookup never performs implicit version substitution.
//! - Batch registration is atomic.
//! - Registry iteration is deterministic.
//! - Removed registrations do not invalidate retained snapshots.
//! - Invalid deserialized or externally constructed metadata is rejected
//!   before registration.
//! - No executable code is loaded or invoked.
//! - No unsafe Rust or arbitrary hardware ceiling is introduced.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::frontend::semantic::domains::ai::{
    AiOperationKind,
    QualifiedName,
};

use crate::ai::version::AiOperationSetVersion;

// ============================================================================
// Operation declaration
// ============================================================================

/// A validated declaration of an AI computational operation.
///
/// This is descriptive metadata. It is not an executable operation
/// implementation, an AST node, a ZUIR operation, or a backend handle.
///
/// The qualified name and operation category reuse the canonical frontend
/// AI semantic types.
///
/// `schema_revision` identifies the contract revision owned by the relevant
/// AI specification or operation-schema implementation. The registry does
/// not interpret or execute that schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiOperationDescriptor {
    name: QualifiedName,
    kind: AiOperationKind,
    version: AiOperationSetVersion,
    schema_revision: Option<String>,
    description: Option<String>,
}

impl AiOperationDescriptor {
    /// Creates an AI operation declaration.
    ///
    /// The operation name, classification and version are required.
    ///
    /// This constructor validates the local declaration. It does not prove
    /// that the operation has an implementation or that a target can execute
    /// it.
    ///
    /// # Errors
    ///
    /// Returns an error if the qualified name or custom operation category
    /// contains an invalid identity component.
    pub fn new(
        name: QualifiedName,
        kind: AiOperationKind,
        version: AiOperationSetVersion,
    ) -> Result<Self, AiRegistryError> {
        validate_qualified_name(&name)?;
        validate_operation_kind(&kind)?;

        Ok(Self {
            name,
            kind,
            version,
            schema_revision: None,
            description: None,
        })
    }

    /// Attaches an operation schema revision.
    ///
    /// The revision is descriptive metadata. Actual schema lookup and
    /// validation remain the responsibility of the owning schema subsystem.
    ///
    /// # Errors
    ///
    /// Returns an error if the revision is empty or contains control
    /// characters.
    pub fn with_schema_revision(
        mut self,
        revision: impl Into<String>,
    ) -> Result<Self, AiRegistryError> {
        let revision = revision.into();
        validate_metadata_text("schema revision", &revision)?;

        self.schema_revision = Some(revision);
        Ok(self)
    }

    /// Attaches a human-readable operation description.
    ///
    /// Descriptions may contain ordinary whitespace and line breaks.
    ///
    /// # Errors
    ///
    /// Returns an error if the description is empty or contains a NUL
    /// character.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, AiRegistryError> {
        let description = description.into();
        validate_description(&description)?;

        self.description = Some(description);
        Ok(self)
    }

    /// Returns the canonical qualified operation name.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Returns the canonical AI semantic operation category.
    #[must_use]
    pub fn kind(&self) -> &AiOperationKind {
        &self.kind
    }

    /// Returns the exact operation-set version.
    #[must_use]
    pub const fn version(&self) -> AiOperationSetVersion {
        self.version
    }

    /// Returns the optional operation schema revision.
    #[must_use]
    pub fn schema_revision(&self) -> Option<&str> {
        self.schema_revision.as_deref()
    }

    /// Returns the optional human-readable description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Revalidates the declaration.
    ///
    /// This method is intended for validation at trust and persistence
    /// boundaries, including after deserialization by a future adapter.
    ///
    /// # Errors
    ///
    /// Returns a structured error when a local declaration invariant
    /// is violated.
    pub fn validate(&self) -> Result<(), AiRegistryError> {
        validate_qualified_name(&self.name)?;
        validate_operation_kind(&self.kind)?;

        if let Some(revision) = &self.schema_revision {
            validate_metadata_text("schema revision", revision)?;
        }

        if let Some(description) = &self.description {
            validate_description(description)?;
        }

        Ok(())
    }
}

// ============================================================================
// Registry key
// ============================================================================

/// Exact identity of a registered AI operation declaration.
///
/// The version is part of the identity. Different versions of the same
/// qualified operation can coexist without replacing one another.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct AiOperationKey {
    name: QualifiedName,
    version: AiOperationSetVersion,
}

impl AiOperationKey {
    fn from_descriptor(descriptor: &AiOperationDescriptor) -> Self {
        Self {
            name: descriptor.name.clone(),
            version: descriptor.version,
        }
    }
}

// ============================================================================
// Registry errors
// ============================================================================

/// Errors produced by AI operation registry operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AiRegistryError {
    /// The operation namespace is empty or invalid.
    InvalidNamespace {
        /// The supplied namespace.
        namespace: String,
    },

    /// The operation name is empty or invalid.
    InvalidName {
        /// The supplied operation name.
        name: String,
    },

    /// A custom operation category contains an invalid qualified identity.
    InvalidCustomKind {
        /// The supplied namespace.
        namespace: String,

        /// The supplied name.
        name: String,
    },

    /// A metadata field is empty or contains invalid characters.
    InvalidMetadata {
        /// The metadata field.
        field: &'static str,
    },

    /// An exact operation registration already exists.
    DuplicateOperation {
        /// Qualified operation identity.
        operation: String,

        /// Exact operation-set version.
        version: AiOperationSetVersion,
    },

    /// The exact requested operation registration does not exist.
    UnknownOperation {
        /// Qualified operation identity.
        operation: String,

        /// Exact requested operation-set version.
        version: AiOperationSetVersion,
    },
}

impl fmt::Display for AiRegistryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNamespace { namespace } => write!(
                formatter,
                "invalid AI operation namespace {namespace:?}"
            ),

            Self::InvalidName { name } => write!(
                formatter,
                "invalid AI operation name {name:?}"
            ),

            Self::InvalidCustomKind { namespace, name } => write!(
                formatter,
                "invalid custom AI operation category {namespace:?}::{name:?}"
            ),

            Self::InvalidMetadata { field } => write!(
                formatter,
                "invalid AI operation metadata field: {field}"
            ),

            Self::DuplicateOperation {
                operation,
                version,
            } => write!(
                formatter,
                "AI operation {operation:?} version {version} is already registered"
            ),

            Self::UnknownOperation {
                operation,
                version,
            } => write!(
                formatter,
                "AI operation {operation:?} version {version} is not registered"
            ),
        }
    }
}

impl Error for AiRegistryError {}

// ============================================================================
// Validation helpers
// ============================================================================

fn validate_identity_component(
    value: &str,
) -> bool {
    !value.trim().is_empty()
        && !value.chars().any(char::is_control)
}

fn validate_qualified_name(
    name: &QualifiedName,
) -> Result<(), AiRegistryError> {
    if !validate_identity_component(&name.namespace) {
        return Err(AiRegistryError::InvalidNamespace {
            namespace: name.namespace.clone(),
        });
    }

    if !validate_identity_component(&name.name) {
        return Err(AiRegistryError::InvalidName {
            name: name.name.clone(),
        });
    }

    Ok(())
}

fn validate_operation_kind(
    kind: &AiOperationKind,
) -> Result<(), AiRegistryError> {
    if let AiOperationKind::Custom(name) = kind {
        if !validate_identity_component(&name.namespace)
            || !validate_identity_component(&name.name)
        {
            return Err(AiRegistryError::InvalidCustomKind {
                namespace: name.namespace.clone(),
                name: name.name.clone(),
            });
        }
    }

    Ok(())
}

fn validate_metadata_text(
    field: &'static str,
    value: &str,
) -> Result<(), AiRegistryError> {
    if !validate_identity_component(value) {
        return Err(AiRegistryError::InvalidMetadata { field });
    }

    Ok(())
}

fn validate_description(
    value: &str,
) -> Result<(), AiRegistryError> {
    if value.trim().is_empty() || value.contains('\0') {
        return Err(AiRegistryError::InvalidMetadata {
            field: "description",
        });
    }

    Ok(())
}

// ============================================================================
// AI operation registry
// ============================================================================

/// Deterministic registry of validated AI operation declarations.
///
/// The registry stores immutable declaration snapshots and performs exact
/// versioned lookup.
///
/// It contains no executable callbacks, hardware handles, global state,
/// network clients, or runtime dependencies.
///
/// Cloning a registry clones its index and shares immutable declarations
/// through `Arc`. Subsequent mutation of either registry does not mutate
/// the other's index.
#[derive(Debug, Clone, Default)]
pub struct AiOperationRegistry {
    operations: BTreeMap<
        AiOperationKey,
        Arc<AiOperationDescriptor>,
    >,
}

impl AiOperationRegistry {
    /// Creates an empty AI operation registry.
    ///
    /// No operation is implicitly registered. The language implementation
    /// or an explicitly authorized extension loader must supply declarations.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of exact operation registrations.
    ///
    /// Different versions of one operation count as separate registrations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the registry has no registrations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Registers one AI operation declaration.
    ///
    /// Exact duplicate registrations are rejected. A different version of
    /// the same operation may be registered independently.
    ///
    /// The returned `Arc` is an immutable snapshot. Removing the registration
    /// later does not invalidate this value.
    ///
    /// # Errors
    ///
    /// Returns a validation error or `DuplicateOperation`.
    pub fn register(
        &mut self,
        descriptor: AiOperationDescriptor,
    ) -> Result<Arc<AiOperationDescriptor>, AiRegistryError> {
        descriptor.validate()?;

        let key = AiOperationKey::from_descriptor(&descriptor);

        if self.operations.contains_key(&key) {
            return Err(AiRegistryError::DuplicateOperation {
                operation: qualified_name_string(&descriptor.name),
                version: descriptor.version,
            });
        }

        let descriptor = Arc::new(descriptor);
        self.operations.insert(key, Arc::clone(&descriptor));

        Ok(descriptor)
    }

    /// Atomically registers a batch of AI operation declarations.
    ///
    /// All declarations are validated against a staged copy of the current
    /// index. The registry is replaced only if every declaration is valid
    /// and no exact duplicate exists.
    ///
    /// This also detects duplicates within the supplied batch.
    ///
    /// An empty batch succeeds without changing the registry.
    ///
    /// # Errors
    ///
    /// Returns the first validation or duplicate-registration error.
    /// On failure, the original registry remains unchanged.
    pub fn register_batch<I>(
        &mut self,
        descriptors: I,
    ) -> Result<(), AiRegistryError>
    where
        I: IntoIterator<Item = AiOperationDescriptor>,
    {
        let mut staged = self.operations.clone();

        for descriptor in descriptors {
            descriptor.validate()?;

            let key = AiOperationKey::from_descriptor(&descriptor);

            if staged.contains_key(&key) {
                return Err(AiRegistryError::DuplicateOperation {
                    operation: qualified_name_string(&descriptor.name),
                    version: descriptor.version,
                });
            }

            staged.insert(
                key,
                Arc::new(descriptor),
            );
        }

        self.operations = staged;
        Ok(())
    }

    /// Resolves an operation using its exact qualified name and version.
    ///
    /// This method never chooses another version automatically.
    ///
    /// # Errors
    ///
    /// Returns `UnknownOperation` when the exact registration is absent.
    pub fn resolve(
        &self,
        name: &QualifiedName,
        version: AiOperationSetVersion,
    ) -> Result<Arc<AiOperationDescriptor>, AiRegistryError> {
        let key = AiOperationKey {
            name: name.clone(),
            version,
        };

        self.operations
            .get(&key)
            .cloned()
            .ok_or_else(|| AiRegistryError::UnknownOperation {
                operation: qualified_name_string(name),
                version,
            })
    }

    /// Looks up an exact operation registration without producing an error.
    #[must_use]
    pub fn get(
        &self,
        name: &QualifiedName,
        version: AiOperationSetVersion,
    ) -> Option<&Arc<AiOperationDescriptor>> {
        self.operations.get(&AiOperationKey {
            name: name.clone(),
            version,
        })
    }

    /// Returns whether an exact operation registration exists.
    #[must_use]
    pub fn contains(
        &self,
        name: &QualifiedName,
        version: AiOperationSetVersion,
    ) -> bool {
        self.operations.contains_key(&AiOperationKey {
            name: name.clone(),
            version,
        })
    }

    /// Iterates over all registered declarations in deterministic order.
    ///
    /// Ordering is by qualified operation name and then exact version.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &Arc<AiOperationDescriptor>> {
        self.operations.values()
    }

    /// Iterates over all registered versions of one operation.
    ///
    /// Versions are returned in ascending version order.
    pub fn versions_for<'a>(
        &'a self,
        name: &'a QualifiedName,
    ) -> impl Iterator<Item = AiOperationSetVersion> + 'a {
        self.operations
            .keys()
            .filter(move |key| &key.name == name)
            .map(|key| key.version)
    }

    /// Removes one exact operation registration.
    ///
    /// Other versions of the same operation remain registered.
    ///
    /// Any previously returned `Arc` remains a valid immutable snapshot.
    pub fn remove(
        &mut self,
        name: &QualifiedName,
        version: AiOperationSetVersion,
    ) -> Option<Arc<AiOperationDescriptor>> {
        self.operations.remove(&AiOperationKey {
            name: name.clone(),
            version,
        })
    }

    /// Returns an immutable snapshot of the current registry index.
    ///
    /// The snapshot can be retained independently of later registry
    /// registration or removal.
    #[must_use]
    pub fn snapshot(&self) -> Self {
        self.clone()
    }
}

fn qualified_name_string(
    name: &QualifiedName,
) -> String {
    format!("{}::{}", name.namespace, name.name)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation_name() -> QualifiedName {
        QualifiedName::new(
            "zamani.ai",
            "inference",
        )
        .expect("valid operation name")
    }

    fn operation(
        version: AiOperationSetVersion,
    ) -> AiOperationDescriptor {
        AiOperationDescriptor::new(
            operation_name(),
            AiOperationKind::Inference,
            version,
        )
        .expect("valid operation descriptor")
    }

    fn version(
        major: u32,
        minor: u32,
        patch: u32,
    ) -> AiOperationSetVersion {
        AiOperationSetVersion::new(
            major,
            minor,
            patch,
        )
    }

    #[test]
    fn registry_starts_empty() {
        let registry = AiOperationRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn registers_and_resolves_exact_operation() {
        let mut registry = AiOperationRegistry::new();
        let operation_version = version(1, 0, 0);

        registry
            .register(operation(operation_version))
            .expect("registration succeeds");

        let resolved = registry
            .resolve(&operation_name(), operation_version)
            .expect("exact operation exists");

        assert_eq!(resolved.name(), &operation_name());
        assert_eq!(resolved.version(), operation_version);
        assert_eq!(resolved.kind(), &AiOperationKind::Inference);
    }

    #[test]
    fn rejects_exact_duplicate_registration() {
        let mut registry = AiOperationRegistry::new();
        let operation_version = version(1, 0, 0);

        registry
            .register(operation(operation_version))
            .expect("first registration succeeds");

        let result = registry.register(operation(operation_version));

        assert!(matches!(
            result,
            Err(AiRegistryError::DuplicateOperation { .. })
        ));

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn allows_multiple_versions_of_one_operation() {
        let mut registry = AiOperationRegistry::new();

        registry
            .register(operation(version(1, 0, 0)))
            .expect("first version");

        registry
            .register(operation(version(2, 0, 0)))
            .expect("second version");

        assert_eq!(registry.len(), 2);

        assert!(registry.contains(
            &operation_name(),
            version(1, 0, 0),
        ));

        assert!(registry.contains(
            &operation_name(),
            version(2, 0, 0),
        ));
    }

    #[test]
    fn exact_lookup_does_not_substitute_versions() {
        let mut registry = AiOperationRegistry::new();

        registry
            .register(operation(version(2, 0, 0)))
            .expect("registration succeeds");

        assert!(matches!(
            registry.resolve(
                &operation_name(),
                version(1, 0, 0),
            ),
            Err(AiRegistryError::UnknownOperation { .. })
        ));
    }

    #[test]
    fn unknown_operation_is_explicit() {
        let registry = AiOperationRegistry::new();

        assert!(matches!(
            registry.resolve(
                &operation_name(),
                version(1, 0, 0),
            ),
            Err(AiRegistryError::UnknownOperation { .. })
        ));
    }

    #[test]
    fn batch_registration_is_atomic() {
        let mut registry = AiOperationRegistry::new();

        registry
            .register(operation(version(1, 0, 0)))
            .expect("initial registration");

        let result = registry.register_batch(vec![
            operation(version(2, 0, 0)),
            operation(version(1, 0, 0)),
        ]);

        assert!(matches!(
            result,
            Err(AiRegistryError::DuplicateOperation { .. })
        ));

        // The first staged entry must not have been published.
        assert_eq!(registry.len(), 1);

        assert!(!registry.contains(
            &operation_name(),
            version(2, 0, 0),
        ));
    }

    #[test]
    fn batch_registration_accepts_distinct_versions() {
        let mut registry = AiOperationRegistry::new();

        registry
            .register_batch(vec![
                operation(version(1, 0, 0)),
                operation(version(1, 1, 0)),
                operation(version(2, 0, 0)),
            ])
            .expect("all versions are distinct");

        assert_eq!(registry.len(), 3);
    }

    #[test]
    fn versions_are_returned_in_deterministic_order() {
        let mut registry = AiOperationRegistry::new();

        registry
            .register_batch(vec![
                operation(version(2, 0, 0)),
                operation(version(1, 2, 0)),
                operation(version(1, 0, 0)),
            ])
            .expect("registration succeeds");

        let versions: Vec<_> = registry
            .versions_for(&operation_name())
            .collect();

        assert_eq!(
            versions,
            vec![
                version(1, 0, 0),
                version(1, 2, 0),
                version(2, 0, 0),
            ]
        );
    }

    #[test]
    fn removal_affects_only_the_exact_version() {
        let mut registry = AiOperationRegistry::new();

        registry
            .register(operation(version(1, 0, 0)))
            .expect("first version");

        registry
            .register(operation(version(2, 0, 0)))
            .expect("second version");

        let removed = registry.remove(
            &operation_name(),
            version(1, 0, 0),
        );

        assert!(removed.is_some());

        assert!(!registry.contains(
            &operation_name(),
            version(1, 0, 0),
        ));

        assert!(registry.contains(
            &operation_name(),
            version(2, 0, 0),
        ));
    }

    #[test]
    fn retained_snapshots_survive_removal() {
        let mut registry = AiOperationRegistry::new();

        let snapshot = registry
            .register(operation(version(1, 0, 0)))
            .expect("registration succeeds");

        registry.remove(
            &operation_name(),
            version(1, 0, 0),
        );

        assert_eq!(snapshot.version(), version(1, 0, 0));
        assert!(registry.is_empty());
    }

    #[test]
    fn registry_snapshot_is_independent() {
        let mut registry = AiOperationRegistry::new();

        registry
            .register(operation(version(1, 0, 0)))
            .expect("registration succeeds");

        let snapshot = registry.snapshot();

        registry.remove(
            &operation_name(),
            version(1, 0, 0),
        );

        assert_eq!(registry.len(), 0);
        assert_eq!(snapshot.len(), 1);
    }

    #[test]
    fn custom_operation_categories_are_supported() {
        let custom_kind = AiOperationKind::Custom(
            QualifiedName::new(
                "future.neuromorphic",
                "event_inference",
            )
            .expect("valid custom category"),
        );

        let descriptor = AiOperationDescriptor::new(
            QualifiedName::new(
                "future.neuromorphic",
                "spike_operation",
            )
            .expect("valid operation name"),
            custom_kind.clone(),
            version(1, 0, 0),
        )
        .expect("custom operation is valid");

        assert_eq!(descriptor.kind(), &custom_kind);
    }

    #[test]
    fn schema_revision_and_description_are_preserved() {
        let descriptor = operation(version(1, 0, 0))
            .with_schema_revision("ai.inference.v1")
            .expect("valid schema revision")
            .with_description("AI inference operation")
            .expect("valid description");

        assert_eq!(
            descriptor.schema_revision(),
            Some("ai.inference.v1")
        );

        assert_eq!(
            descriptor.description(),
            Some("AI inference operation")
        );
    }

    #[test]
    fn invalid_metadata_is_rejected() {
        assert!(
            operation(version(1, 0, 0))
                .with_schema_revision("")
                .is_err()
        );

        assert!(
            operation(version(1, 0, 0))
                .with_description("   ")
                .is_err()
        );
    }

    #[test]
    fn empty_batch_is_a_valid_no_op() {
        let mut registry = AiOperationRegistry::new();

        registry
            .register_batch(Vec::new())
            .expect("empty batch succeeds");

        assert!(registry.is_empty());
    }

    #[test]
    fn iteration_is_deterministic() {
        let mut registry = AiOperationRegistry::new();

        let first = QualifiedName::new(
            "zamani.ai",
            "training",
        )
        .expect("valid name");

        let second = QualifiedName::new(
            "zamani.ai",
            "inference",
        )
        .expect("valid name");

        registry
            .register_batch(vec![
                AiOperationDescriptor::new(
                    first,
                    AiOperationKind::Training,
                    version(1, 0, 0),
                )
                .expect("valid descriptor"),
                AiOperationDescriptor::new(
                    second,
                    AiOperationKind::Inference,
                    version(1, 0, 0),
                )
                .expect("valid descriptor"),
            ])
            .expect("registration succeeds");

        let names: Vec<_> = registry
            .iter()
            .map(|entry| entry.name().name.as_str())
            .collect();

        assert_eq!(names, vec!["inference", "training"]);
    }
}
