//! Zamani backend registry.
//!
//! Path: `src/backend/registry.rs`
//! Minimum Rust version: 1.97
//! Safety: safe Rust only; no `unsafe`.
//!
//! # Responsibility
//!
//! Owns registration, discovery, canonical identifier lookup, alias
//! resolution, and construction of backend implementations.
//!
//! This module does not:
//! - choose a target based on source semantics;
//! - perform capability negotiation or target feasibility analysis;
//! - compile or lower intermediate representations;
//! - execute external tools;
//! - write artifacts to disk;
//! - implement LLVM, WebAssembly, native, quantum, or HDL code generation;
//! - maintain a second compiler or semantic model.
//!
//! # Integration contract
//!
//! The parent `backend` module must expose the existing canonical backend
//! interface as `super::Backend`. Its contract currently consists of:
//!
//! - `target_name(&self) -> &str`;
//! - `generate(&self, module: &IrModule) -> Result<String, CodeGenError>`;
//! - `file_extension(&self) -> &str`.
//!
//! The trait remains owned by the canonical backend API. Do not introduce
//! another competing `Backend` trait.
//!
//! The canonical module root must re-export or define that interface after
//! the existing `src/backend.rs` versus `src/backend/mod.rs` conflict has
//! been resolved.
//!
//! Target selection must consume the target and capability contracts
//! established by the surrounding backend architecture. This registry
//! deliberately does not guess which target is suitable for a program.
//!
//! # Invariants
//!
//! - Canonical identifiers are unique.
//! - Aliases are globally unique across canonical identifiers and aliases.
//! - Registration either succeeds completely or changes no registry state.
//! - Enumeration is deterministic and sorted by canonical identifier.
//! - A factory must produce an implementation with the registered canonical
//!   target name.
//! - Factory execution happens outside the registry lock.
//! - The registry does not impose a universal target-count limit.
//! - No fallback backend is selected when a lookup fails.
//! - Registry poisoning is reported rather than silently ignored.
//!
//! # Extension policy
//!
//! A new target is added by registering its descriptor and factory.
//! New targets must implement the existing backend interface and separately
//! satisfy the compatibility, capability, artifact, and verification
//! contracts of the complete backend system.
//!
//! Registration alone does not establish that a target is production-ready.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, RwLock};

use super::Backend;

/// Canonical identifier used to refer to a registered backend.
///
/// Identifiers are case-sensitive. They are not filesystem paths, display
/// labels, architecture enums, or capability declarations.
pub type BackendId = String;

/// A backend factory that can be called concurrently.
///
/// Factories should be inexpensive, deterministic where applicable, and
/// free of unexpected filesystem, console, or process side effects.
pub type BackendFactory =
    Arc<dyn Fn() -> Box<dyn Backend> + Send + Sync + 'static>;

/// Public metadata for a registered backend.
///
/// Metadata is deliberately independent of concrete backend implementation
/// types so that callers can discover targets without constructing them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendDescriptor {
    /// Globally unique, canonical, case-sensitive identifier.
    pub id: BackendId,

    /// Additional names that resolve to `id`.
    pub aliases: Vec<String>,

    /// Human-readable description.
    pub description: String,

    /// Advertised output extension, such as `.s`, `.wat`, or `.ll`.
    ///
    /// This is descriptive metadata, not proof that the output is valid
    /// assembly, WebAssembly, or LLVM IR.
    pub file_extension: String,
}

impl BackendDescriptor {
    /// Construct a descriptor with no aliases or description.
    pub fn new(
        id: impl Into<String>,
        file_extension: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            aliases: Vec::new(),
            description: String::new(),
            file_extension: file_extension.into(),
        }
    }

    /// Add an alias.
    #[must_use]
    pub fn with_alias(mut self, alias: impl Into<String>) -> Self {
        self.aliases.push(alias.into());
        self
    }

    /// Add a human-readable description.
    #[must_use]
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = description.into();
        self
    }
}

/// Errors produced by registry operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    /// A canonical identifier, alias, or other required field is invalid.
    InvalidIdentifier {
        field: &'static str,
        value: String,
        reason: &'static str,
    },

    /// The descriptor contains the same alias more than once.
    DuplicateAliasInDescriptor {
        backend_id: String,
        alias: String,
    },

    /// A canonical identifier is already registered.
    DuplicateBackend {
        backend_id: String,
    },

    /// An alias conflicts with an existing canonical identifier.
    IdentifierConflictsWithAlias {
        identifier: String,
        existing_backend_id: String,
    },

    /// An alias is already owned by another backend.
    AliasAlreadyRegistered {
        alias: String,
        existing_backend_id: String,
    },

    /// The requested identifier or alias cannot be resolved.
    BackendNotFound {
        identifier: String,
    },

    /// A constructed backend does not identify itself as registered.
    FactoryIdentityMismatch {
        registered_id: String,
        actual_id: String,
    },

    /// The registry lock was poisoned by a previous panic.
    LockPoisoned {
        operation: &'static str,
    },
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier {
                field,
                value,
                reason,
            } => write!(
                f,
                "invalid backend {field} `{value}`: {reason}"
            ),

            Self::DuplicateAliasInDescriptor {
                backend_id,
                alias,
            } => write!(
                f,
                "backend `{backend_id}` declares alias `{alias}` more than once"
            ),

            Self::DuplicateBackend { backend_id } => write!(
                f,
                "backend `{backend_id}` is already registered"
            ),

            Self::IdentifierConflictsWithAlias {
                identifier,
                existing_backend_id,
            } => write!(
                f,
                "identifier `{identifier}` conflicts with an alias owned by \
                 backend `{existing_backend_id}`"
            ),

            Self::AliasAlreadyRegistered {
                alias,
                existing_backend_id,
            } => write!(
                f,
                "alias `{alias}` is already registered for \
                 backend `{existing_backend_id}`"
            ),

            Self::BackendNotFound { identifier } => write!(
                f,
                "no backend is registered for identifier or alias `{identifier}`"
            ),

            Self::FactoryIdentityMismatch {
                registered_id,
                actual_id,
            } => write!(
                f,
                "backend factory registered as `{registered_id}` produced \
                 backend `{actual_id}`"
            ),

            Self::LockPoisoned { operation } => write!(
                f,
                "backend registry lock is poisoned during `{operation}`"
            ),
        }
    }
}

impl Error for RegistryError {}

/// A successfully registered descriptor and its factory.
#[derive(Clone)]
struct Registration {
    descriptor: BackendDescriptor,
    factory: BackendFactory,
}

/// Internal registry state.
///
/// `BTreeMap` provides stable identifier ordering independently of
/// registration order, hash randomization, or process address layout.
#[derive(Default)]
struct RegistryState {
    backends: BTreeMap<BackendId, Registration>,
    aliases: BTreeMap<String, BackendId>,
}

/// Thread-safe registry of backend implementations.
///
/// Cloning this value shares the same registry. It does not create an
/// independent copy of registrations.
///
/// The lock protects only registry metadata. Backend factories execute
/// after the lock has been released.
#[derive(Clone, Default)]
pub struct BackendRegistry {
    state: Arc<RwLock<RegistryState>>,
}

impl BackendRegistry {
    /// Create an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a backend descriptor and factory atomically.
    ///
    /// No registry state is changed if validation fails or if the
    /// descriptor conflicts with an existing registration.
    ///
    /// This method does not execute the factory. Actual implementation
    /// construction is deferred until [`Self::create`] is called.
    pub fn register<F>(
        &self,
        descriptor: BackendDescriptor,
        factory: F,
    ) -> Result<(), RegistryError>
    where
        F: Fn() -> Box<dyn Backend> + Send + Sync + 'static,
    {
        validate_descriptor(&descriptor)?;

        let mut aliases = descriptor.aliases.clone();
        aliases.sort();
        aliases.dedup();

        if aliases.len() != descriptor.aliases.len() {
            let mut seen = std::collections::BTreeSet::new();

            for alias in &descriptor.aliases {
                if !seen.insert(alias) {
                    return Err(RegistryError::DuplicateAliasInDescriptor {
                        backend_id: descriptor.id.clone(),
                        alias: alias.clone(),
                    });
                }
            }
        }

        let mut state = self
            .state
            .write()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "register",
            })?;

        if state.backends.contains_key(&descriptor.id) {
            return Err(RegistryError::DuplicateBackend {
                backend_id: descriptor.id,
            });
        }

        // Validate every potential conflict before making any changes.
        // A failed registration must never leave partial aliases behind.
        if let Some(existing_backend_id) =
            state.aliases.get(&descriptor.id)
        {
            return Err(RegistryError::IdentifierConflictsWithAlias {
                identifier: descriptor.id,
                existing_backend_id: existing_backend_id.clone(),
            });
        }

        for alias in &aliases {
            if alias == &descriptor.id {
                return Err(RegistryError::DuplicateAliasInDescriptor {
                    backend_id: descriptor.id.clone(),
                    alias: alias.clone(),
                });
            }

            if state.backends.contains_key(alias) {
                return Err(RegistryError::IdentifierConflictsWithAlias {
                    identifier: alias.clone(),
                    existing_backend_id: alias.clone(),
                });
            }

            if let Some(existing_backend_id) = state.aliases.get(alias) {
                return Err(RegistryError::AliasAlreadyRegistered {
                    alias: alias.clone(),
                    existing_backend_id: existing_backend_id.clone(),
                });
            }
        }

        let id = descriptor.id.clone();

        let registration = Registration {
            descriptor,
            factory: Arc::new(factory),
        };

        // All conflict checks have passed. Commit the registration while
        // holding the write lock so concurrent registrations cannot race.
        for alias in aliases {
            state.aliases.insert(alias, id.clone());
        }

        state.backends.insert(id, registration);

        Ok(())
    }

    /// Return whether an identifier or alias resolves to a registered
    /// backend.
    pub fn contains(
        &self,
        identifier: &str,
    ) -> Result<bool, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "contains",
            })?;

        Ok(state.backends.contains_key(identifier)
            || state.aliases.contains_key(identifier))
    }

    /// Resolve a canonical identifier or alias.
    ///
    /// The returned identifier is always canonical.
    pub fn resolve(
        &self,
        identifier: &str,
    ) -> Result<BackendId, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "resolve",
            })?;

        if state.backends.contains_key(identifier) {
            return Ok(identifier.to_owned());
        }

        if let Some(canonical_id) = state.aliases.get(identifier) {
            return Ok(canonical_id.clone());
        }

        Err(RegistryError::BackendNotFound {
            identifier: identifier.to_owned(),
        })
    }

    /// Return a registered descriptor using its canonical identifier or
    /// an alias.
    pub fn descriptor(
        &self,
        identifier: &str,
    ) -> Result<BackendDescriptor, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "descriptor",
            })?;

        let canonical_id = if state.backends.contains_key(identifier) {
            identifier
        } else {
            state
                .aliases
                .get(identifier)
                .map(String::as_str)
                .ok_or_else(|| RegistryError::BackendNotFound {
                    identifier: identifier.to_owned(),
                })?
        };

        state
            .backends
            .get(canonical_id)
            .map(|registration| registration.descriptor.clone())
            .ok_or_else(|| RegistryError::BackendNotFound {
                identifier: identifier.to_owned(),
            })
    }

    /// Construct a fresh backend instance.
    ///
    /// The factory is invoked without holding the registry lock. This
    /// permits independent backend construction and avoids deadlocks if
    /// construction indirectly consults the registry.
    ///
    /// The returned implementation must report the registered canonical
    /// identifier through `Backend::target_name`.
    pub fn create(
        &self,
        identifier: &str,
    ) -> Result<Box<dyn Backend>, RegistryError> {
        let (canonical_id, factory) = {
            let state = self
                .state
                .read()
                .map_err(|_| RegistryError::LockPoisoned {
                    operation: "create",
                })?;

            let canonical_id =
                if state.backends.contains_key(identifier) {
                    identifier.to_owned()
                } else {
                    state
                        .aliases
                        .get(identifier)
                        .cloned()
                        .ok_or_else(|| RegistryError::BackendNotFound {
                            identifier: identifier.to_owned(),
                        })?
                };

            let registration = state
                .backends
                .get(&canonical_id)
                .ok_or_else(|| RegistryError::BackendNotFound {
                    identifier: identifier.to_owned(),
                })?;

            (
                canonical_id,
                Arc::clone(&registration.factory),
            )
        };

        let backend = factory();
        let actual_id = backend.target_name().to_owned();

        if actual_id != canonical_id {
            return Err(RegistryError::FactoryIdentityMismatch {
                registered_id: canonical_id,
                actual_id,
            });
        }

        Ok(backend)
    }

    /// Return all registered descriptors in canonical identifier order.
    pub fn descriptors(
        &self,
    ) -> Result<Vec<BackendDescriptor>, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "descriptors",
            })?;

        Ok(state
            .backends
            .values()
            .map(|registration| registration.descriptor.clone())
            .collect())
    }

    /// Return all canonical backend identifiers in deterministic order.
    pub fn backend_ids(&self) -> Result<Vec<BackendId>, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "backend_ids",
            })?;

        Ok(state.backends.keys().cloned().collect())
    }

    /// Return the number of registered canonical backends.
    pub fn len(&self) -> Result<usize, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "len",
            })?;

        Ok(state.backends.len())
    }

    /// Return whether the registry contains no backends.
    pub fn is_empty(&self) -> Result<bool, RegistryError> {
        Ok(self.len()? == 0)
    }

    /// Unregister a backend by canonical identifier or alias.
    ///
    /// This removes the registry entry and all aliases owned by that
    /// entry. Already constructed backend instances remain valid and
    /// independently owned by their callers.
    ///
    /// Unregistration is intended for controlled lifecycle management,
    /// tests, and dynamically managed extensions. Production code should
    /// normally finalize built-in registrations during initialization.
    pub fn unregister(
        &self,
        identifier: &str,
    ) -> Result<BackendDescriptor, RegistryError> {
        let mut state = self
            .state
            .write()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "unregister",
            })?;

        let canonical_id =
            if state.backends.contains_key(identifier) {
                identifier.to_owned()
            } else {
                state
                    .aliases
                    .get(identifier)
                    .cloned()
                    .ok_or_else(|| RegistryError::BackendNotFound {
                        identifier: identifier.to_owned(),
                    })?
            };

        let registration = state
            .backends
            .remove(&canonical_id)
            .ok_or_else(|| RegistryError::BackendNotFound {
                identifier: identifier.to_owned(),
            })?;

        for alias in &registration.descriptor.aliases {
            state.aliases.remove(alias);
        }

        Ok(registration.descriptor)
    }
}

/// Validate metadata without consulting the registry.
///
/// This function contains no architecture-specific target list. Future
/// targets can use new identifiers without changing this module.
fn validate_descriptor(
    descriptor: &BackendDescriptor,
) -> Result<(), RegistryError> {
    validate_identifier("id", &descriptor.id)?;

    if descriptor.file_extension.trim().is_empty() {
        return Err(RegistryError::InvalidIdentifier {
            field: "file extension",
            value: descriptor.file_extension.clone(),
            reason: "must not be empty",
        });
    }

    if descriptor.file_extension.chars().any(char::is_control)
        || descriptor.file_extension.contains('/')
        || descriptor.file_extension.contains('\\')
    {
        return Err(RegistryError::InvalidIdentifier {
            field: "file extension",
            value: descriptor.file_extension.clone(),
            reason: "must be metadata, not a path or control sequence",
        });
    }

    if descriptor.description.chars().any(char::is_control) {
        return Err(RegistryError::InvalidIdentifier {
            field: "description",
            value: descriptor.description.clone(),
            reason: "must not contain control characters",
        });
    }

    for alias in &descriptor.aliases {
        validate_identifier("alias", alias)?;
    }

    Ok(())
}

/// Validate a canonical identifier or alias.
///
/// Qualified names such as `native.aarch64`, `quantum.qasm`, and
/// vendor-specific names are permitted. This is intentionally not a
/// closed enum of known architectures or computing domains.
fn validate_identifier(
    field: &'static str,
    value: &str,
) -> Result<(), RegistryError> {
    if value.is_empty() || value.trim() != value {
        return Err(RegistryError::InvalidIdentifier {
            field,
            value: value.to_owned(),
            reason: "must be non-empty and have no surrounding whitespace",
        });
    }

    if value.chars().any(char::is_control)
        || value.chars().any(char::is_whitespace)
        || value.contains('/')
        || value.contains('\\')
    {
        return Err(RegistryError::InvalidIdentifier {
            field,
            value: value.to_owned(),
            reason: "must not contain whitespace, path separators, or control characters",
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestBackend {
        id: &'static str,
    }

    impl Backend for TestBackend {
        fn target_name(&self) -> &str {
            self.id
        }

        fn generate(
            &self,
            _module: &crate::ir_gen::IrModule,
        ) -> Result<String, super::super::CodeGenError> {
            Ok(String::new())
        }

        fn file_extension(&self) -> &str {
            ".test"
        }
    }

    fn descriptor(id: &str) -> BackendDescriptor {
        BackendDescriptor::new(id, ".test")
    }

    fn register_test_backend(
        registry: &BackendRegistry,
        id: &'static str,
        metadata: BackendDescriptor,
    ) -> Result<(), RegistryError> {
        registry.register(metadata, move || {
            Box::new(TestBackend { id })
        })
    }

    #[test]
    fn registers_and_constructs_backend() {
        let registry = BackendRegistry::new();

        register_test_backend(
            &registry,
            "native.test",
            descriptor("native.test").with_alias("test-native"),
        )
        .expect("registration should succeed");

        let backend = registry
            .create("test-native")
            .expect("alias should construct backend");

        assert_eq!(backend.target_name(), "native.test");
        assert_eq!(
            registry.resolve("test-native").unwrap(),
            "native.test"
        );
    }

    #[test]
    fn rejects_duplicate_canonical_identifier() {
        let registry = BackendRegistry::new();

        register_test_backend(
            &registry,
            "native.test",
            descriptor("native.test"),
        )
        .unwrap();

        let result = register_test_backend(
            &registry,
            "native.test",
            descriptor("native.test"),
        );

        assert!(matches!(
            result,
            Err(RegistryError::DuplicateBackend { .. })
        ));

        assert_eq!(registry.len().unwrap(), 1);
    }

    #[test]
    fn rejects_alias_collision_without_partial_registration() {
        let registry = BackendRegistry::new();

        register_test_backend(
            &registry,
            "native.first",
            descriptor("native.first").with_alias("shared"),
        )
        .unwrap();

        let result = register_test_backend(
            &registry,
            "native.second",
            descriptor("native.second")
                .with_alias("new-alias")
                .with_alias("shared"),
        );

        assert!(matches!(
            result,
            Err(RegistryError::AliasAlreadyRegistered { .. })
        ));

        assert!(!registry.contains("native.second").unwrap());
        assert!(!registry.contains("new-alias").unwrap());
        assert_eq!(registry.len().unwrap(), 1);
    }

    #[test]
    fn rejects_alias_that_conflicts_with_canonical_identifier() {
        let registry = BackendRegistry::new();

        register_test_backend(
            &registry,
            "native.first",
            descriptor("native.first"),
        )
        .unwrap();

        let result = register_test_backend(
            &registry,
            "native.second",
            descriptor("native.second").with_alias("native.first"),
        );

        assert!(matches!(
            result,
            Err(RegistryError::IdentifierConflictsWithAlias { .. })
        ));

        assert_eq!(registry.len().unwrap(), 1);
    }

    #[test]
    fn rejects_duplicate_aliases_in_one_descriptor() {
        let registry = BackendRegistry::new();

        let metadata = descriptor("native.test")
            .with_alias("duplicate")
            .with_alias("duplicate");

        let result = register_test_backend(
            &registry,
            "native.test",
            metadata,
        );

        assert!(matches!(
            result,
            Err(RegistryError::DuplicateAliasInDescriptor { .. })
        ));

        assert!(registry.is_empty().unwrap());
    }

    #[test]
    fn rejects_factory_identity_mismatch() {
        let registry = BackendRegistry::new();

        registry
            .register(descriptor("native.expected"), || {
                Box::new(TestBackend { id: "native.actual" })
            })
            .unwrap();

        let result = registry.create("native.expected");

        assert!(matches!(
            result,
            Err(RegistryError::FactoryIdentityMismatch { .. })
        ));
    }

    #[test]
    fn enumeration_is_deterministic() {
        let registry = BackendRegistry::new();

        for id in ["zeta", "alpha", "middle"] {
            register_test_backend(
                &registry,
                match id {
                    "zeta" => "zeta",
                    "alpha" => "alpha",
                    _ => "middle",
                },
                descriptor(id),
            )
            .unwrap();
        }

        assert_eq!(
            registry.backend_ids().unwrap(),
            vec!["alpha", "middle", "zeta"]
        );
    }

    #[test]
    fn aliases_resolve_to_canonical_descriptor() {
        let registry = BackendRegistry::new();

        register_test_backend(
            &registry,
            "quantum.qasm",
            descriptor("quantum.qasm")
                .with_alias("qasm")
                .with_description("Quantum assembly backend"),
        )
        .unwrap();

        let metadata = registry.descriptor("qasm").unwrap();

        assert_eq!(metadata.id, "quantum.qasm");
        assert_eq!(metadata.aliases, vec!["qasm"]);
    }

    #[test]
    fn unregister_removes_aliases() {
        let registry = BackendRegistry::new();

        register_test_backend(
            &registry,
            "native.test",
            descriptor("native.test").with_alias("test"),
        )
        .unwrap();

        let removed = registry.unregister("test").unwrap();

        assert_eq!(removed.id, "native.test");
        assert!(!registry.contains("test").unwrap());
        assert!(!registry.contains("native.test").unwrap());
        assert!(registry.is_empty().unwrap());
    }

    #[test]
    fn missing_backend_is_an_explicit_error() {
        let registry = BackendRegistry::new();

        assert!(matches!(
            registry.create("missing.target"),
            Err(RegistryError::BackendNotFound { .. })
        ));
    }

    #[test]
    fn cloned_registry_shares_registrations() {
        let registry = BackendRegistry::new();
        let other_handle = registry.clone();

        register_test_backend(
            &registry,
            "native.test",
            descriptor("native.test"),
        )
        .unwrap();

        assert!(other_handle.contains("native.test").unwrap());
    }

    #[test]
    fn rejects_invalid_identifiers() {
        let registry = BackendRegistry::new();

        for id in ["", " native", "native ", "native target", "a/b"] {
            let result = register_test_backend(
                &registry,
                "native.test",
                descriptor(id),
            );

            assert!(
                matches!(result, Err(RegistryError::InvalidIdentifier { .. })),
                "expected invalid identifier error for {id:?}"
            );
        }

        assert!(registry.is_empty().unwrap());
    }

    #[test]
    fn supports_extensible_qualified_identifiers() {
        let registry = BackendRegistry::new();
        let id = "future.domain.vendor.target";

        register_test_backend(
            &registry,
            "future.domain.vendor.target",
            descriptor(id),
        )
        .unwrap();

        assert_eq!(registry.resolve(id).unwrap(), id);
    }
}